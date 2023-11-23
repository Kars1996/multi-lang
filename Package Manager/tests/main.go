package main

import (
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestFetchPackageInfo(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte(`{
			"name": "test-package",
			"version": "1.0.0",
			"author": "Test Author",
			"dependencies": {
				"dependency1": "1.0.0",
				"dependency2": "2.0.0"
			}
		}`))
	}))
	defer server.Close()

	CDNURL = server.URL

	packageName := "test-package"
	packageVersion := "1.0.0"

	pkg, err := fetchPackageInfo(packageName, packageVersion)
	if err != nil {
		t.Fatalf("fetchPackageInfo error: %v", err)
	}

	expected := &Package{
		Name:         "test-package",
		Version:      "1.0.0",
		Author:       "Test Author",
		Dependencies: map[string]string{"dependency1": "1.0.0", "dependency2": "2.0.0"},
	}

	if *pkg != *expected {
		t.Errorf("Expected package %+v, got %+v", expected, pkg)
	}
}
