// @generated
/// Implement [`DataProvider<ExtendedPictographicV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_extpict_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_EXTPICT_V1: &'static <icu_properties::provider::ExtendedPictographicV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xA9\0\0\0\xAA\0\0\0\xAE\0\0\0\xAF\0\0\0< \0\0= \0\0I \0\0J \0\0\"!\0\0#!\0\09!\0\0:!\0\0\x94!\0\0\x9A!\0\0\xA9!\0\0\xAB!\0\0\x1A#\0\0\x1C#\0\0(#\0\0)#\0\0\x88#\0\0\x89#\0\0\xCF#\0\0\xD0#\0\0\xE9#\0\0\xF4#\0\0\xF8#\0\0\xFB#\0\0\xC2$\0\0\xC3$\0\0\xAA%\0\0\xAC%\0\0\xB6%\0\0\xB7%\0\0\xC0%\0\0\xC1%\0\0\xFB%\0\0\xFF%\0\0\0&\0\0\x06&\0\0\x07&\0\0\x13&\0\0\x14&\0\0\x86&\0\0\x90&\0\0\x06'\0\0\x08'\0\0\x13'\0\0\x14'\0\0\x15'\0\0\x16'\0\0\x17'\0\0\x1D'\0\0\x1E'\0\0!'\0\0\"'\0\0('\0\0)'\0\x003'\0\x005'\0\0D'\0\0E'\0\0G'\0\0H'\0\0L'\0\0M'\0\0N'\0\0O'\0\0S'\0\0V'\0\0W'\0\0X'\0\0c'\0\0h'\0\0\x95'\0\0\x98'\0\0\xA1'\0\0\xA2'\0\0\xB0'\0\0\xB1'\0\0\xBF'\0\0\xC0'\0\x004)\0\x006)\0\0\x05+\0\0\x08+\0\0\x1B+\0\0\x1D+\0\0P+\0\0Q+\0\0U+\0\0V+\0\x0000\0\x0010\0\0=0\0\0>0\0\0\x972\0\0\x982\0\0\x992\0\0\x9A2\0\0\0\xF0\x01\0\0\xF1\x01\0\r\xF1\x01\0\x10\xF1\x01\0/\xF1\x01\x000\xF1\x01\0l\xF1\x01\0r\xF1\x01\0~\xF1\x01\0\x80\xF1\x01\0\x8E\xF1\x01\0\x8F\xF1\x01\0\x91\xF1\x01\0\x9B\xF1\x01\0\xAD\xF1\x01\0\xE6\xF1\x01\0\x01\xF2\x01\0\x10\xF2\x01\0\x1A\xF2\x01\0\x1B\xF2\x01\0/\xF2\x01\x000\xF2\x01\x002\xF2\x01\0;\xF2\x01\0<\xF2\x01\0@\xF2\x01\0I\xF2\x01\0\xFB\xF3\x01\0\0\xF4\x01\0>\xF5\x01\0F\xF5\x01\0P\xF6\x01\0\x80\xF6\x01\0\0\xF7\x01\0t\xF7\x01\0\x80\xF7\x01\0\xD5\xF7\x01\0\0\xF8\x01\0\x0C\xF8\x01\0\x10\xF8\x01\0H\xF8\x01\0P\xF8\x01\0Z\xF8\x01\0`\xF8\x01\0\x88\xF8\x01\0\x90\xF8\x01\0\xAE\xF8\x01\0\0\xF9\x01\0\x0C\xF9\x01\0;\xF9\x01\0<\xF9\x01\0F\xF9\x01\0G\xF9\x01\0\0\xFB\x01\0\0\xFC\x01\0\xFE\xFF\x01\0") }, 3537usize)
            });
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::ExtendedPictographicV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::ExtendedPictographicV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    if locale.is_empty() {
                        Ok(Self::SINGLETON_PROPS_EXTPICT_V1)
                    } else {
                        Err(icu_provider::DataErrorKind::ExtraneousLocale)
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::ExtendedPictographicV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
