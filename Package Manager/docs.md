
# KPM (Kars Package Manager)

KPM is a simple package manager written in Go, designed to install and manage packages from a CDN (Content Delivery Network) server.

## Features

- Install packages from a CDN server.
- Support for both KPM and NPM (Node Package Manager) packages.
- Dependency resolution and installation.
- Progress bar for package installation.
- Color-coded console output for a visually appealing experience.

## Usage

### Installation

Before using KPM, make sure you have Go installed on your system.

```bash
go install kpm.go
```

### Installing Packages

To install a package using KPM, use the following syntax:

```bash
kpm install <package-name>*@<version>
```

Replace `<package-name>` with the name of the package, and `<version>` with the desired version.

### Examples

```bash
# Install a KPM package
kpm install my-package@1.0.0

# Install an NPM package
kpm install npm-package@2.5.1
```

## Package Structure

KPM expects packages to be structured on the CDN server as follows:

- CDN_URL/packages/<package-name>/package.js
- CDN_URL/packages/<package-name>/package.json

The `package.json` file contains metadata such as version, author, dependencies, and package name.

## Dependencies

KPM uses the `github.com/cheggaaa/pb/v3` package for the progress bar. If not installed, KPM will attempt to install it automatically.

## Contributing

Contributions to KPM are welcome. Feel free to submit bug reports, feature requests, or pull requests on the [GitHub repository](https://github.com/kars1996/kpm).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
