// @generated
/// Implement [`DataProvider<SoftDottedV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_sd_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_SD_V1: &'static <icu_properties::provider::SoftDottedV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"i\0\0\0k\0\0\0/\x01\0\x000\x01\0\0I\x02\0\0J\x02\0\0h\x02\0\0i\x02\0\0\x9D\x02\0\0\x9E\x02\0\0\xB2\x02\0\0\xB3\x02\0\0\xF3\x03\0\0\xF4\x03\0\0V\x04\0\0W\x04\0\0X\x04\0\0Y\x04\0\0b\x1D\0\0c\x1D\0\0\x96\x1D\0\0\x97\x1D\0\0\xA4\x1D\0\0\xA5\x1D\0\0\xA8\x1D\0\0\xA9\x1D\0\0-\x1E\0\0.\x1E\0\0\xCB\x1E\0\0\xCC\x1E\0\0q \0\0r \0\0H!\0\0J!\0\0|,\0\0},\0\0\"\xD4\x01\0$\xD4\x01\0V\xD4\x01\0X\xD4\x01\0\x8A\xD4\x01\0\x8C\xD4\x01\0\xBE\xD4\x01\0\xC0\xD4\x01\0\xF2\xD4\x01\0\xF4\xD4\x01\0&\xD5\x01\0(\xD5\x01\0Z\xD5\x01\0\\\xD5\x01\0\x8E\xD5\x01\0\x90\xD5\x01\0\xC2\xD5\x01\0\xC4\xD5\x01\0\xF6\xD5\x01\0\xF8\xD5\x01\0*\xD6\x01\0,\xD6\x01\0^\xD6\x01\0`\xD6\x01\0\x92\xD6\x01\0\x94\xD6\x01\0\x1A\xDF\x01\0\x1B\xDF\x01\0L\xE0\x01\0N\xE0\x01\0h\xE0\x01\0i\xE0\x01\0") }, 50usize)
            });
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::SoftDottedV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::SoftDottedV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    if locale.is_empty() {
                        Ok(Self::SINGLETON_PROPS_SD_V1)
                    } else {
                        Err(icu_provider::DataErrorKind::ExtraneousLocale)
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::SoftDottedV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
