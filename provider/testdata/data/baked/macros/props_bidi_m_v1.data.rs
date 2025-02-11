// @generated
/// Implement [`DataProvider<BidiMirroredV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_bidi_m_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_BIDI_M_V1: &'static <icu_properties::provider::BidiMirroredV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"(\0\0\0*\0\0\0<\0\0\0=\0\0\0>\0\0\0?\0\0\0[\0\0\0\\\0\0\0]\0\0\0^\0\0\0{\0\0\0|\0\0\0}\0\0\0~\0\0\0\xAB\0\0\0\xAC\0\0\0\xBB\0\0\0\xBC\0\0\0:\x0F\0\0>\x0F\0\0\x9B\x16\0\0\x9D\x16\0\09 \0\0; \0\0E \0\0G \0\0} \0\0\x7F \0\0\x8D \0\0\x8F \0\0@!\0\0A!\0\0\x01\"\0\0\x05\"\0\0\x08\"\0\0\x0E\"\0\0\x11\"\0\0\x12\"\0\0\x15\"\0\0\x17\"\0\0\x1A\"\0\0\x1E\"\0\0\x1F\"\0\0#\"\0\0$\"\0\0%\"\0\0&\"\0\0'\"\0\0+\"\0\x004\"\0\09\"\0\0:\"\0\0;\"\0\0M\"\0\0R\"\0\0V\"\0\0_\"\0\0a\"\0\0b\"\0\0c\"\0\0d\"\0\0l\"\0\0n\"\0\0\x8D\"\0\0\x8F\"\0\0\x93\"\0\0\x98\"\0\0\x99\"\0\0\xA2\"\0\0\xA4\"\0\0\xA6\"\0\0\xB9\"\0\0\xBE\"\0\0\xC0\"\0\0\xC9\"\0\0\xCE\"\0\0\xD0\"\0\0\xD2\"\0\0\xD6\"\0\0\xEE\"\0\0\xF0\"\0\0\0#\0\0\x08#\0\0\x0C#\0\0 #\0\0\"#\0\0)#\0\0+#\0\0h'\0\0v'\0\0\xC0'\0\0\xC1'\0\0\xC3'\0\0\xC7'\0\0\xC8'\0\0\xCA'\0\0\xCB'\0\0\xCE'\0\0\xD3'\0\0\xD7'\0\0\xDC'\0\0\xDF'\0\0\xE2'\0\0\xF0'\0\0\x83)\0\0\x99)\0\0\x9B)\0\0\xA1)\0\0\xA2)\0\0\xB0)\0\0\xB8)\0\0\xB9)\0\0\xC0)\0\0\xC6)\0\0\xC9)\0\0\xCA)\0\0\xCE)\0\0\xD3)\0\0\xD4)\0\0\xD6)\0\0\xD8)\0\0\xDD)\0\0\xE1)\0\0\xE2)\0\0\xE3)\0\0\xE6)\0\0\xE8)\0\0\xEA)\0\0\xF4)\0\0\xFA)\0\0\xFC)\0\0\xFE)\0\0\n*\0\0\x1D*\0\0\x1E*\0\0\"*\0\0$*\0\0%*\0\0&*\0\0'*\0\0)*\0\0**\0\0+*\0\0/*\0\x004*\0\x006*\0\0<*\0\0?*\0\0W*\0\0Y*\0\0d*\0\0f*\0\0j*\0\0n*\0\0o*\0\0q*\0\0s*\0\0u*\0\0y*\0\0\xA4*\0\0\xA6*\0\0\xAE*\0\0\xAF*\0\0\xD7*\0\0\xDC*\0\0\xDD*\0\0\xDE*\0\0\xDF*\0\0\xE2*\0\0\xE7*\0\0\xEC*\0\0\xEF*\0\0\xF3*\0\0\xF4*\0\0\xF7*\0\0\xFC*\0\0\xFD*\0\0\xFE*\0\0\xFE+\0\0\xFF+\0\0\x02.\0\0\x06.\0\0\t.\0\0\x0B.\0\0\x0C.\0\0\x0E.\0\0\x1C.\0\0\x1E.\0\0 .\0\0*.\0\0U.\0\0].\0\0\x080\0\0\x120\0\0\x140\0\0\x1C0\0\0Y\xFE\0\0_\xFE\0\0d\xFE\0\0f\xFE\0\0\x08\xFF\0\0\n\xFF\0\0\x1C\xFF\0\0\x1D\xFF\0\0\x1E\xFF\0\0\x1F\xFF\0\0;\xFF\0\0<\xFF\0\0=\xFF\0\0>\xFF\0\0[\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0_\xFF\0\0a\xFF\0\0b\xFF\0\0d\xFF\0\0\xDB\xD6\x01\0\xDC\xD6\x01\0\x15\xD7\x01\0\x16\xD7\x01\0O\xD7\x01\0P\xD7\x01\0\x89\xD7\x01\0\x8A\xD7\x01\0\xC3\xD7\x01\0\xC4\xD7\x01\0") }, 553usize)
            });
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::BidiMirroredV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::BidiMirroredV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    if locale.is_empty() {
                        Ok(Self::SINGLETON_PROPS_BIDI_M_V1)
                    } else {
                        Err(icu_provider::DataErrorKind::ExtraneousLocale)
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::BidiMirroredV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
