# Changelog

## [0.5.1](https://github.com/lakekeeper/lakekeeper/compare/v0.5.0...v0.5.1) (2024-12-12)


### Features

* **openapi:** document error models in openapi ([#658](https://github.com/lakekeeper/lakekeeper/issues/658)) ([2a67196](https://github.com/lakekeeper/lakekeeper/commit/2a67196a9f9844db0f846cb2e9016c4d4620b0b5))


### Miscellaneous Chores

* release 0.5.1 ([f8aa87c](https://github.com/lakekeeper/lakekeeper/commit/f8aa87ca8b7a8074389cd43a39007b2652a46494))

## [0.5.0](https://github.com/lakekeeper/lakekeeper/compare/v0.4.3...v0.5.0) (2024-12-06)


### ⚠ BREAKING CHANGES

* Default to single-tenant / single-project with NIL Project-ID

### Features

* Add Iceberg REST Spec to swagger ([2eaa10e](https://github.com/lakekeeper/lakekeeper/commit/2eaa10e7cb233282fe4452bf526deee7c07a5fb5))
* Create default Project on Bootstrap ([2eaa10e](https://github.com/lakekeeper/lakekeeper/commit/2eaa10e7cb233282fe4452bf526deee7c07a5fb5))
* Default to single-tenant / single-project with NIL Project-ID ([2eaa10e](https://github.com/lakekeeper/lakekeeper/commit/2eaa10e7cb233282fe4452bf526deee7c07a5fb5))
* Fine Grained Access Controls with OpenFGA ([2eaa10e](https://github.com/lakekeeper/lakekeeper/commit/2eaa10e7cb233282fe4452bf526deee7c07a5fb5))
* Hierarchical Namespaces ([2eaa10e](https://github.com/lakekeeper/lakekeeper/commit/2eaa10e7cb233282fe4452bf526deee7c07a5fb5))
* Optionally return uuids for Iceberg APIs ([2eaa10e](https://github.com/lakekeeper/lakekeeper/commit/2eaa10e7cb233282fe4452bf526deee7c07a5fb5))
* Project Management APIs ([2eaa10e](https://github.com/lakekeeper/lakekeeper/commit/2eaa10e7cb233282fe4452bf526deee7c07a5fb5))
* Server Info Endpoint ([2eaa10e](https://github.com/lakekeeper/lakekeeper/commit/2eaa10e7cb233282fe4452bf526deee7c07a5fb5))
* split table metadata into tables ([#478](https://github.com/lakekeeper/lakekeeper/issues/478)) ([942fa97](https://github.com/lakekeeper/lakekeeper/commit/942fa97c98049d15a50168ce7d7a9e711d9de3d1))


### Miscellaneous Chores

* release 0.5.0 ([b1b2ee6](https://github.com/lakekeeper/lakekeeper/commit/b1b2ee6d0f068adf9a60719c1cfb88201825d389))

## [0.4.3](https://github.com/lakekeeper/lakekeeper/compare/v0.4.2...v0.4.3) (2024-11-13)


### Miscellaneous Chores

* release 0.4.3 ([e577ab2](https://github.com/lakekeeper/lakekeeper/commit/e577ab2e4da78d612e87bd4844307c28098e2c31))

## [0.4.2](https://github.com/lakekeeper/lakekeeper/compare/v0.4.1...v0.4.2) (2024-10-28)


### Miscellaneous Chores

* release 0.4.2 ([1d8c469](https://github.com/lakekeeper/lakekeeper/commit/1d8c469cd30121e17455b2c2a13e9f0a46f7f630))

## [0.4.1](https://github.com/lakekeeper/lakekeeper/compare/v0.4.0...v0.4.1) (2024-10-15)


### Bug Fixes

* bug in join for listing view representations ([d2f1d7a](https://github.com/lakekeeper/lakekeeper/commit/d2f1d7aad9497f8bf4fc04d8347949bf25ffc16a))
* gcs integration test are now running in ci ([d2f1d7a](https://github.com/lakekeeper/lakekeeper/commit/d2f1d7aad9497f8bf4fc04d8347949bf25ffc16a))
* increase keycloak timeout in integration tests ([d2f1d7a](https://github.com/lakekeeper/lakekeeper/commit/d2f1d7aad9497f8bf4fc04d8347949bf25ffc16a))
* purge tests are now properly executed in ci ([d2f1d7a](https://github.com/lakekeeper/lakekeeper/commit/d2f1d7aad9497f8bf4fc04d8347949bf25ffc16a))

## [0.4.0](https://github.com/lakekeeper/lakekeeper/compare/v0.3.0...v0.4.0) (2024-10-03)


### Features

* **cache:** cache metadata location in signer ([#334](https://github.com/lakekeeper/lakekeeper/issues/334)) ([fa0863c](https://github.com/lakekeeper/lakekeeper/commit/fa0863cdbf5df626eec083499d76add4dade4e0b))
* New TableMetadataBuilder with: ID Reassignments, Metadata expiry, safe binding... ([#387](https://github.com/lakekeeper/lakekeeper/issues/387)) ([e5c1c77](https://github.com/lakekeeper/lakekeeper/commit/e5c1c77fced957cd6703e1ae6ec77e151414a63e))
* **storage:** support for google cloud storage (gcs) ([#361](https://github.com/lakekeeper/lakekeeper/issues/361)) ([ebb4e27](https://github.com/lakekeeper/lakekeeper/commit/ebb4e27f729e20e30f87e5ce4c2d2351c2422ca6))

## [0.3.0](https://github.com/lakekeeper/lakekeeper/compare/v0.2.1...v0.3.0) (2024-08-26)


### Features

* Add support for custom Locations for Namespaces & Tables ([1d2ac6f](https://github.com/lakekeeper/lakekeeper/commit/1d2ac6f4b3910bf161c47d0224689b6e611d15ab))
* **aws:** sts credentials for s3 ([#236](https://github.com/lakekeeper/lakekeeper/issues/236)) ([dbf775b](https://github.com/lakekeeper/lakekeeper/commit/dbf775b6e226a8b8822f2e725ec317b4230aa0c4))

## [0.2.1](https://github.com/lakekeeper/lakekeeper/compare/v0.2.0...v0.2.1) (2024-07-29)


### Miscellaneous Chores

* release 0.2.1 ([587ea12](https://github.com/lakekeeper/lakekeeper/commit/587ea129780c21a3cd0fa8dd371b6901dede4c20))

## [0.2.0](https://github.com/lakekeeper/lakekeeper/compare/v0.1.0...v0.2.0) (2024-07-26)


### Features

* **views:** commit views ([#146](https://github.com/lakekeeper/lakekeeper/issues/146)) ([0f6310b](https://github.com/lakekeeper/lakekeeper/commit/0f6310b2486cc608af6844c35be7a45ebeb998cd))

## [0.1.0](https://github.com/lakekeeper/lakekeeper/compare/v0.1.0-rc3...v0.1.0) (2024-06-17)


### Miscellaneous Chores

* 🚀 Release 0.1.0 ([a5def9a](https://github.com/lakekeeper/lakekeeper/commit/a5def9a527aa615779b60fe8fc5a18aaa47f33ee))

## [0.1.0-rc3](https://github.com/lakekeeper/lakekeeper/compare/v0.1.0-rc2...v0.1.0-rc3) (2024-06-17)


### Miscellaneous Chores

* 🚀 Release 0.1.0-rc3 ([9b0d219](https://github.com/lakekeeper/lakekeeper/commit/9b0d219e865dce85803fc93da7233e92d3e8b4b8))

## [0.1.0-rc2](https://github.com/lakekeeper/lakekeeper/compare/v0.1.0-rc1...v0.1.0-rc2) (2024-06-17)


### Miscellaneous Chores

* 🚀 Release 0.1.0-rc2 ([9bc25ef](https://github.com/lakekeeper/lakekeeper/commit/9bc25ef2b44d6c29556a5d0913c076904b1cb010))

## [0.1.0-rc1](https://github.com/lakekeeper/lakekeeper/compare/v0.0.2-rc1...v0.1.0-rc1) (2024-06-16)


### Miscellaneous Chores

* 🚀 Release 0.1.0-rc1 ([ba6e5d5](https://github.com/lakekeeper/lakekeeper/commit/ba6e5d5c8a59cb1da5b61dd559c783998559debf))

## [0.0.2-rc1](https://github.com/lakekeeper/lakekeeper/compare/v0.0.1...v0.0.2-rc1) (2024-06-16)


### Miscellaneous Chores

* 🚀 Release 0.0.2-rc1 ([eb34b9c](https://github.com/lakekeeper/lakekeeper/commit/eb34b9cd613bb2d72d4a9b33b103d36c7649bd57))

## [0.0.1](https://github.com/lakekeeper/lakekeeper/compare/v0.0.0...v0.0.1) (2024-06-15)


### Miscellaneous Chores

* 🚀 Release 0.0.1 ([c52ddec](https://github.com/lakekeeper/lakekeeper/commit/c52ddec7520ec16ed0b6f70c5e3108a7d8a35665))
