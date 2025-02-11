// @generated
/// Implement [`DataProvider<CanonicalCombiningClassNameToValueV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_from_ccc_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_FROM_CCC_V1: &'static <icu_properties::provider::CanonicalCombiningClassNameToValueV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::names::PropertyValueNameToEnumMapV1 {
                map: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"O\0\0\0\0\0\x01\0\x06\0\x10\0\x1B\0\x1D\0\x1F\0\"\0&\0)\0-\0;\0O\0]\0p\0q\0v\0\x80\0\x8B\0\x8D\0\x8F\0\x94\0\x9A\0\xA0\0\xA5\0\xAB\0\xB0\0\xB6\0\xBC\0\xC1\0\xC7\0\xCD\0\xD3\0\xD8\0\xDD\0\xE2\0\xE7\0\xEC\0\xF1\0\xF6\0\xFB\0\0\x01\x05\x01\n\x01\x0F\x01\x14\x01\x19\x01\x1E\x01#\x01(\x01-\x012\x017\x01<\x01A\x01F\x01K\x01P\x01R\x01T\x01`\x01l\x01p\x01{\x01\x89\x01\x8B\x01\x97\x01\x99\x01\x9A\x01\x9E\x01\xA0\x01\xAD\x01\xAF\x01\xB4\x01\xB6\x01\xBD\x01\xBE\x01\xC3\x01\xC9\x01AAboveAbove_LeftAbove_RightALARATAATARATBATBLAttached_AboveAttached_Above_RightAttached_BelowAttached_Below_LeftBBelowBelow_LeftBelow_RightBLBRCCC10CCC103CCC107CCC11CCC118CCC12CCC122CCC129CCC13CCC130CCC132CCC133CCC14CCC15CCC16CCC17CCC18CCC19CCC20CCC21CCC22CCC23CCC24CCC25CCC26CCC27CCC28CCC29CCC30CCC31CCC32CCC33CCC34CCC35CCC36CCC84CCC91DADBDouble_AboveDouble_BelowHANRHan_ReadingIota_SubscriptISKana_VoicingKVLLeftNKNot_ReorderedNRNuktaOVOverlayRRightViramaVR") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE6\0\xE6\0\xE4\0\xE8\0\xE4\0\xE8\0\xD6\0\xD8\0\xCA\0\xC8\0\xD6\0\xD8\0\xCA\0\xC8\0\xDC\0\xDC\0\xDA\0\xDE\0\xDA\0\xDE\0\n\0g\0k\0\x0B\0v\0\x0C\0z\0\x81\0\r\0\x82\0\x84\0\x85\0\x0E\0\x0F\0\x10\0\x11\0\x12\0\x13\0\x14\0\x15\0\x16\0\x17\0\x18\0\x19\0\x1A\0\x1B\0\x1C\0\x1D\0\x1E\0\x1F\0 \0!\0\"\0#\0$\0T\0[\0\xEA\0\xE9\0\xEA\0\xE9\0\x06\0\x06\0\xF0\0\xF0\0\x08\0\x08\0\xE0\0\xE0\0\x07\0\0\0\0\0\x07\0\x01\0\x01\0\xE2\0\xE2\0\t\0\t\0") })
                },
            };
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::CanonicalCombiningClassNameToValueV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::CanonicalCombiningClassNameToValueV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    if locale.is_empty() {
                        Ok(Self::SINGLETON_PROPNAMES_FROM_CCC_V1)
                    } else {
                        Err(icu_provider::DataErrorKind::ExtraneousLocale)
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::CanonicalCombiningClassNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
