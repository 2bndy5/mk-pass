import tomllib
from pathlib import Path
from conan import ConanFile
from conan.tools.files import copy
from conan.tools.cmake import CMakeToolchain, CMake, cmake_layout, CMakeDeps


class mk_passRecipe(ConanFile):
    name = "mk_pass"
    package_type = "library"

    # Optional metadata
    topics = ("password", "generator")

    # Binary configuration
    settings = "os", "compiler", "build_type", "arch"
    options = {"shared": [True, False], "fPIC": [True, False]}
    default_options = {"shared": True, "fPIC": True}

    def init(self):
        cargo_toml = tomllib.loads(Path("Cargo.toml").read_text(encoding="utf-8"))
        cargo_pkg = cargo_toml["workspace"]["package"]
        self.version = cargo_pkg["version"]
        self.license = cargo_pkg["license"]
        self.author = cargo_pkg["authors"][0] + " 2bndy5@gmail.com"
        self.url = cargo_pkg["repository"]
        self.description = cargo_pkg["description"]

    def export_sources(self):
        # The path of the CMakeLists.txt and sources we want to export are one level above
        folder = Path(self.recipe_folder).parent.parent
        # Sources are located in the same place as this recipe, copy them to the recipe
        exports_sources = (
            "bindings/cpp/CMakeLists.txt",
            "bindings/cpp/src/*",
            "bindings/cpp/include/*",
            "bindings/cpp/Cargo.toml",
            "Cargo.toml",
            "mk-pass/**",
            # corrosion (in cmake) will try to load the entire cargo workspace, so include unneeded sources too
            "bindings/py/src/*bindings/py/Cargo.toml",
            "bindings/node/src/*bindings/node/Cargo.toml",
            "docs/cargo.toml",
            "docs/src/*.rs",
            "README.md",
            "LICENSE",
        )
        for src in exports_sources:
            copy(self, src, folder, self.export_sources_folder)

    def export(self):
        folder = Path(self.recipe_folder).parent.parent
        # need to export the root Cargo.toml to
        # extract metadata about the package dynamically
        copy(self, "Cargo.toml", folder, self.export_folder)

    def config_options(self):
        if self.settings.os == "Windows":
            self.options.rm_safe("fPIC")

    def configure(self):
        if self.options.shared:
            self.options.rm_safe("fPIC")

    def layout(self):
        cmake_layout(self)
        self.folders.root = "../.."
        self.folders.source = "bindings/cpp"
        self.folders.build = "bindings/cpp/build"

    def generate(self):
        deps = CMakeDeps(self)
        deps.generate()
        tc = CMakeToolchain(self)
        tc.generate()

    def build(self):
        cmake = CMake(self)
        cmake.configure()
        cmake.build()

    def package(self):
        cmake = CMake(self)
        cmake.install()

    def package_info(self):
        self.cpp_info.libs = ["mk_pass"]
