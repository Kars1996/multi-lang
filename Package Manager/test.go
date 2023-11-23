package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"os/exec"
	"strings"
	"sync"

	"github.com/cheggaaa/pb/v3"
	"github.com/fatih/color"
)

type Package struct {
	Name         string            `json:"name"`
	Version      string            `json:"version"`
	Author       string            `json:"author"`
	Dependencies map[string]string `json:"dependencies"`
}

const CDNURL = "https://cdn.kars.bio/packages"

func fetchPackageInfo(packageName, version string) (*Package, error) {
	url := fmt.Sprintf("%s/%s/%s.json", CDNURL, packageName, version)
	resp, err := http.Get(url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("failed to fetch package information: %s", resp.Status)
	}

	var pkg Package
	err = json.NewDecoder(resp.Body).Decode(&pkg)
	if err != nil {
		return nil, err
	}

	return &pkg, nil
}

func downloadPackageCode(packageName, version string, bar *pb.ProgressBar) error {
	url := fmt.Sprintf("%s/%s/%s/package.js", CDNURL, packageName, version)
	resp, err := http.Get(url)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("failed to download package code: %s", resp.Status)
	}

	file, err := os.Create(fmt.Sprintf("packages/%s-%s.js", packageName, version))
	if err != nil {
		return err
	}
	defer file.Close()

	writer := io.MultiWriter(file, bar)

	_, err = io.Copy(writer, resp.Body)
	if err != nil {
		return err
	}

	return nil
}

func savePackage(pkg *Package) {
	filename := fmt.Sprintf("packages/%s-%s.json", pkg.Name, pkg.Version)
	serialized, err := json.MarshalIndent(pkg, "", "  ")
	if err != nil {
		fmt.Println("Failed to serialize package:", err)
		return
	}
	err = os.WriteFile(filename, serialized, 0644)
	if err != nil {
		fmt.Println("Failed to write package file:", err)
	}
}

func installPackage(pkg *Package) {
	fmt.Printf("Installing %s@%s...\n", pkg.Name, pkg.Version)

	bar := pb.New64(0).Set(pb.Bytes, true).SetTemplateString(`{{string . "prefix"}}{{counters . }} {{string . "suffix"}}{{bar . }} {{percent . }}`)
	bar.SetMaxWidth(80)
	bar.Set("prefix", color.BlueString("[Downloading]"))
	bar.Start()

	err := downloadPackageCode(pkg.Name, pkg.Version, bar)
	bar.Finish()
	if err != nil {
		fmt.Println("Failed to download package code:", err)
		return
	}

	simulateInstallation(pkg)
	fmt.Println("Package installed successfully!")
}

func simulateInstallation(pkg *Package) {
	if script, ok := pkg.Dependencies["scripts"]; ok {
		runScript(script)
	}
}

func runScript(script string) {
	fmt.Printf("Running script: %s\n", script)
	cmd := exec.Command("sh", "-c", script)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		fmt.Println("Failed to run script:", err)
	}
}

func fetchAllPackages() ([]string, error) {
	url := fmt.Sprintf("%s/packages.json", CDNURL)
	resp, err := http.Get(url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("failed to fetch package list: %s", resp.Status)
	}

	var packageList []string
	err = json.NewDecoder(resp.Body).Decode(&packageList)
	if err != nil {
		return nil, err
	}

	return packageList, nil
}

func main() {
	args := os.Args
	if len(args) != 4 || args[1] != "install" || args[3] == "" {
		fmt.Println("Usage: kpm install <package-name>*@<version>")
		os.Exit(1)
	}

	packageInfo := strings.Split(args[2], "*")
	if len(packageInfo) != 2 || packageInfo[0] == "" || packageInfo[1] == "" {
		fmt.Println("Invalid package name or version.")
		os.Exit(1)
	}

	packageName := packageInfo[0]
	packageVersion := packageInfo[1]

	allPackages, err := fetchAllPackages()
	if err != nil {
		fmt.Println("Failed to fetch package list:", err)
		os.Exit(1)
	}

	var packageFound bool
	for _, pkg := range allPackages {
		if pkg == packageName {
				packageFound = true
				break
		}
	}

	if !packageFound {
		fmt.Printf("Package %s not found on CDN. Fallback to npm...\n", packageName)
		npmInstall(packageName, packageVersion)
		return
	}

	fmt.Println("Fetching package information from CDN...")
	packageInfo, err := fetchPackageInfo(packageName, packageVersion)
	if err != nil {
		fmt.Println("Failed to fetch package information:", err)
		os.Exit(1)
	}

	savePackage(packageInfo)

	installPackage(packageInfo)
}

func npmInstall(packageName, packageVersion string) {
	fmt.Printf("Installing %s@%s using npm...\n", packageName, packageVersion)
	cmd := exec.Command("npm", "install", fmt.Sprintf("%s@%s", packageName, packageVersion))
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		fmt.Println("Failed to install package using npm:", err)
		os.Exit(1)
	}
	fmt.Println("Package installed successfully using npm")
}