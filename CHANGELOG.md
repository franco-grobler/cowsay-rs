# Changelog

All notable changes to this project will be documented in this file.

## [0.3.12] - 2025-12-30

### 💼 Other

- Set tap
- Add docker to builds

### ⚙️ Miscellaneous Tasks

- Publish cowsay-wasm
- *(release)* Prepare release version v0.3.12

## [0.3.11] - 2025-12-30

### ⚙️ Miscellaneous Tasks

- Fix releases
- *(release)* Prepare release version v0.3.11

## [0.3.10] - 2025-12-30

### ⚙️ Miscellaneous Tasks

- *(release)* Prepare release version v0.3.9
- *(release)* Prepare release version v0.3.10

## [0.3.9] - 2025-12-30

### ⚙️ Miscellaneous Tasks

- *(release)* Prepare release version v0.3.8
- Fix homebrew publish create pr
- *(release)* Prepare release version v0.3.9

## [0.3.8] - 2025-12-30

### ⚙️ Miscellaneous Tasks

- *(release)* Prepare release version v0.3.7
- Fix publish permissions
- *(release)* Prepare release version v0.3.8

## [0.3.7] - 2025-12-30

### ⚙️ Miscellaneous Tasks

- *(release)* Prepare release version v0.3.6
- Homebrew publish permissions
- *(release)* Prepare release version v0.3.7

## [0.3.6] - 2025-12-30

### 💼 Other

- Update release ci builds

### ⚙️ Miscellaneous Tasks

- *(release)* Prepare release version v0.3.5
- *(release)* Prepare release version v0.3.6

## [0.3.5] - 2025-12-30

### ⚙️ Miscellaneous Tasks

- *(release)* Prepare release version v0.3.4
- Use correct yaml extension
- *(release)* Prepare release version v0.3.5

## [0.3.4] - 2025-12-30

### ⚙️ Miscellaneous Tasks

- *(release)* Prepare release version v0.3.3
- Update release
- Cleanup readme
- *(release)* Prepare release version v0.3.4

## [0.3.3] - 2025-12-30

### ⚙️ Miscellaneous Tasks

- *(release)* Prepare release version v0.3.2
- Add attestations for release
- Update dependencies
- *(release)* Prepare release version v0.3.3

## [0.3.2] - 2025-12-28

### 💼 Other

- Fix release workflow

### ⚙️ Miscellaneous Tasks

- *(release)* Prepare release version v0.3.1
- *(release)* Prepare release version v0.3.2

## [0.3.1] - 2025-12-28

### 💼 Other

- Use fixed action versions

### ⚙️ Miscellaneous Tasks

- Update lockfile
- Add script to update ci jobs
- Update checkout version
- *(release)* Prepare release version v0.3.1

## [0.3.0] - 2025-12-26

### 🚀 Features

- *(cows)* Add default cows as per pearl library
- *(parser)* Basic parser complete
- *(template)* Create template lib
- *(template)* Basic template parser complete
- *(template)* Allow creating from template content
- *(template)* Strip escape characters from loaded templates
- *(parser)* Use template package for handling formatting and loading of templates
- *(cowsay)* Create main lib
- *(scripts)* Add script to generate test data
- *(template)* Allow overriding variables
- *(cli)* Add cowsay-rs cli
- Add cargo dist
- *(wasm)* Basic wasm bindings
- *(wasm)* Use option module and initialise wasm module
- *(website)* Switch to bun-nuxt
- *(website)* Add basic playground layout
- *(website)* Add about section
- *(website)* Add placeholder footer
- *(website)* Update seo content
- *(cowsay)* Add list cows method
- *(parser)* Use trimmed text when generating balloon
- *(wasm)* Add list cows method
- *(website)* Add fields
- *(website)* Add cowsay compostable
- *(website)* Add errors compostable
- *(website)* Update playground view
- *(cli)* Add list cows option
- *(website)* Add anchor to view
- *(website)* Basic dark support
- *(website)* Add eslint import sorter
- Use follow tags flag when pushing tags in release script

### 🐛 Bug Fixes

- *(cowsay)* Fallback to default template
- *(template)* Add new line to end of output
- *(cowsay)* Use full file name
- *(loader)* Missing escape character
- *(cows)* Remove cows requiring perl script
- *(parser)* Use usize for balloon width
- *(parser)* Update build to match cow template updates
- *(parser)* Fix multiline parsing
- *(template)* Update description pattern to allow multiline comments
- Drop permissions and checkout action fetch depth from ci workflow
- *(cowsay)* Correct cowsay error implementation
- *(template)* Correct parse error implementation
- *(cli)* Update builder to match cowsay lib updates
- *(cowsay)* Implement error source for cowsay parse error
- *(template)* Implement error source for parse error
- *(cli)* Docstring for file flag
- *(cli)* Linting errors
- *(wasm)* Create a option constructor struct for options constructor
- *(wasm)* Dropped error
- *(wasm)* Add dependency getrandom to fix compilation error
- *(website)* Broken dev site
- *(website)* Favicon type
- *(website)* Drop id fields
- *(parser)* Handle multiline text breaks
- *(wasm)* Wrap column binding
- *(website)* Use string for wrap column data
- *(website)* Update fields styling
- *(website)* Cleanup redundant classes
- *(website)* Update views with links
- *(website)* Ensure wrap is disabled when not selected
- *(website)* Fix dev server and update deps
- Merge conflicts on cargo keys
- *(website)* Build wasm during prebuild
- *(website)* Formatting
- Readme updates
- Release script
- Double v before version

### 💼 Other

- Use workspaces
- Do not publish helper libs
- Fix unnecessary dependencies
- Add npm to publish jobs
- *(website)* Add venodred wasm dependency
- Add containerfile for cli
- *(website)* Add scaffold dockerfile
- Update layer orders for cowsay containers

### 🚜 Refactor

- *(parser)* Use usize for ballon width
- *(cowsay)* Simplify setting wrap
- *(cowsay)* Simplify parsing option
- *(template)* Use once lock for compiling regex
- *(template)* Use once lock for compiling loader regex
- *(template)* Use default variable when loading from file or template
- *(template)* Simplify template rendering
- *(parser)* Simplify build
- *(cowsay)* Ensure get random cow will not cause panics
- *(parser)* Use string references for cow builder
- *(parser)* Use string references for cow
- *(cowsay)* Use string references
- *(parser)* Use string struct for building template
- *(cowsay)* Ensure no panics will occur with get cow from file
- *(cowsay)* Simplify random cow lookup
- *(cowsay)* Remove unnecessary clone
- *(wasm)* Simplify builder construction

### 📚 Documentation

- Add documentation missing documentation
- *(cowsay)* Add builder docs
- *(cowsay)* Add cows docs
- *(cowsay)* Add library docs
- *(cli)* Add missing docstrings
- *(wasm)* Add missing docstrings
- Fix bad docs
- *(cowsay)* Fix examples

### 🧪 Testing

- *(cowsay)* Add expected output text files
- *(parser)* Fix tests after updates
- *(cowsay)* Add compatibility tests
- *(cowsay)* Add file compatibility tests
- *(cowsay)* Fix file compatibility tests
- *(cowsay)* Add test for long inputs
- Add nextest config
- Use pretty assertions for checking templates
- *(cowsay)* Update tests to match new implementation
- *(cli)* Add integration tests for cli options
- *(cli)* Fix random test
- *(cowsay)* Fix linting issues
- *(wasm)* Create api test module
- *(cli)* Update test to reflect more complex secanrio

### ⚙️ Miscellaneous Tasks

- Init
- *(cows)* Drop daemon cow
- *(parser)* Cleanup prints
- Cleanup workspace
- Clean up gitignore
- Add rustfmt config file
- *(parser)* Use defaults from template
- *(template)* Cleanup tests
- Add cowsay lib to dependencies
- *(cowsay)* Format errors
- Fix spelling and remove prints
- Fix rust version
- Update rust version on dev shell
- Bump rust version
- *(parser)* Format lib
- *(cowsay)* Format tests
- *(cowsay)* Format cows file
- *(template)* Remove dead code
- Update formatting and linting config
- Add nextest and cargo-llvm-cov for coverage collection
- Add comprehensive ci file from ruff
- Add cargo shear to dev environment
- Cleanup
- Drop determine changes
- Add pre commit to shell
- Drop windows tests
- Add ci badge
- Add python version variable
- Force add pre commit config
- Do not hide precommit config file
- Add missing scopes
- *(template)* Fix typo in default cow const
- Fix runner for release test
- Add dependabot config
- Add toolchain and linting files
- Format ci workflow
- Add security workflow
- Add cargo audit to dev environment
- Add components to rus toolchain config
- Update timeouts
- Fix security workflow
- Fix prettier config
- Update security workflow
- Add descriptions to all crates
- Add release workflow from dist
- Update devenv lockfile and formatting
- Add codeql workflow
- Disable javascript until website is merged
- Fix cargo toml metadata
- Update toolchain
- *(cowsay)* Fix linting
- Use workspace version
- *(cowsay)* Fix lint errors
- Hide unused dependencies
- Add apple file markers to gitignore
- Add wasm target
- Add wasm ci flows
- *(website)* Scaffold project
- *(website)* Formatting
- Remove bad taste cows
- Add assets
- Update ignore file
- *(website)* Add ci workflows for website
- Remove untrack files
- *(website)* Cleanup entrypoint
- *(website)* Cleanup
- *(website)* Install bun
- *(website)* Set working directory
- *(website)* Add cache for bun installs
- *(website)* Linting fixes
- Formatting
- *(website)* Update name for website deployment
- *(website)* Add cowsay-wasm package
- Add website code scanning
- Add default ci permission
- Add dependabot checks for website
- Build website to only run on website changes
- Add website permissions
- *(website)* Formatting
- Formatting
- *(website)* Bump dun version
- Formatting
- Fix ci workflow
- Fix dependency path
- *(template)* Fix linting issues
- Fix deployment and formatting
- Add readme and update rust version
- Add git cliff with config
- Add release workflow
- Add typos to packages
- Fix version release workflow
- Add readme
- Add nix badge to readme
- Add recipes for running and building cowsay containers
- Update devenv lockfile
- Run version release on main
- Install typos cli in release version workflow
- Add output env
- Rename changelog workflow
- Fix changelog generation
- Update precommit config scopes
- Rename script
- Add release script
- Update release config and recipe
- *(release)* Prepare release version v0.3.0

<!-- generated by git-cliff -->
