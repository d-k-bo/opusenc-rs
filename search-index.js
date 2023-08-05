var searchIndex = JSON.parse('{\
"opusenc":{"doc":"High-level bindings for libopusenc.","t":"NNNNNNNNNNNNNNNNNNNNNNNNNNDNNNNNNNNNNNNNNNNNDNNENNNNNNNNNNNNNNNNNNNNNNNNENNNNNNNNNNNNNNNNNNNNNNNENNNNNNNNNEENNNENNNNNNNNNNNNNGNNNNNNNNNNNNNNNNNNNFLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLCLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLFLL","n":["ASIN","AcoustID","AcoustIDFingerprint","Album","Album","AlbumArtist","AlbumArtistSortOrder","AlbumSortOrder","AllocFail","Arranger","Artist","Artist","Artist","ArtistLogotype","ArtistSortOrder","Artists","BPM","BackCover","BadArg","BandOrchestra","Barcode","BrightColoredFish","CannotOpen","CatalogNumber","CloseFail","Comment","Comments","Compilation","Composer","Composer","ComposerSortOrder","Conductor","Conductor","Contact","Copyright","Copyright","Date","Description","Director","DiscNumber","DiscSubtitle","DuringPerformance","DuringRecording","EncodedBy","Encoder","EncoderSettings","Engineer","Error","FileIcon32","FrontCover","Genre","Genre","Grouping","Illustration","InitialKey","InternalError","InvalidIcon","InvalidPicture","InvalidString","InvalidValue","Isrc","Isrc","Language","LeadArtist","LeafletPage","License","License","Location","Location","Lyricist","Lyricist","Lyrics","MappingFamily","Media","Media","MixDJ","Mixer","MonoStereo","Mood","Movement","MovementCount","MovementNumber","MusicBrainzArtistID","MusicBrainzDiscID","MusicBrainzOriginalArtistID","MusicBrainzOriginalReleaseID","MusicBrainzRecordingID","MusicBrainzReleaseArtistID","MusicBrainzReleaseGroupID","MusicBrainzReleaseID","MusicBrainzTRMID","MusicBrainzTrackID","MusicBrainzWorkID","MusicIPFingerprint","MusicIPPUID","Opusenc","OpusencError","Organization","OriginalFilename","OriginalReleaseDate","OriginalReleaseYear","Other","Other","OtherFileIcon","Performer","Performer","PicardTag","PictureType","Producer","PublisherLogotype","Rating","RecommendedTag","RecordLabel","ReleaseCountry","ReleaseDate","ReleaseStatus","ReleaseType","Remixer","ReplayGainAlbumGain","ReplayGainAlbumPeak","ReplayGainAlbumRange","ReplayGainReferenceLoudness","ReplayGainTrackGain","ReplayGainTrackPeak","ReplayGainTrackRange","Result","ScreenCapture","Script","ShowWorkMovement","Subtitle","Surround","Title","TooLate","TotalDiscs","TotalTracks","TrackNumber","TrackNumber","TrackTitle","TrackTitleSortOrder","Unimplemented","Version","Website","WorkTitle","WriteFail","Writer","abi_version","add","add_picture","add_picture_from_memory","add_string","as_cstr","as_cstr","as_ref","as_ref","as_ref","as_ref","as_str","as_str","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","chain_current","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","continue_new_file","create","create_file","default","default","drain","drop","drop","eq","eq","ffi","flush_header","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from_primitive","into","into","into","into","into","into","into","into","provide","provide","ptr","ptr","source","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","version_str","write","write_float"],"q":[[0,"opusenc"]],"d":["","","","The collection name to which this track belongs","","","","","","","Artist/performer","The artist generally considered responsible for the work. …","","Band/artist logotype","","","","Cover (back)","","Band/Orchestra","","A bright colored fish","","","","","Comments to be attached to an Ogg Opus file.","","Composer","","","Conductor","","Contact information for the creators or distributors of …","Copyright attribution, e.g., ‘2001 Nobody’s Band’ or …","","Date the track was recorded","A short text description of the contents","","","","During performance","During recording","","Encoder for creating Ogg Opus files.","","","The error type returned by all library functions.","32x32 pixel ‘file icon’ (PNG only)","Cover (front)","A short text indication of music genre","","","Illustration","","","","","The provided string contains a nul byte.","libopusenc returned an invalid value for an option.","ISRC number for the track; see the ISRC intro page for …","","","Lead artist/lead performer/soloist","Leaflet page","License information, for example, ‘All Rights Reserved’…","","Recording location","Location where track was recorded","Lyricist/text writer","","","Describes wether the channels are mapped as mono, stereo …","Media (e.g., label side of a CD)","","","","","","","","","","","","","","","","","","","","","","libopusenc returned an error.","Error code as returned from libopusenc.","Name of the organization producing the track (i.e. the ‘…","","","","Other","","Other file icon","The artist(s) who performed the work. In classical music …","","Common tags used by Musicbrainz Picard.","Type of an attached picture.","","Publisher/studio logotype","","Tags recommended by the Ogg Vorbis specification.","","","","","","","","","","","","","","","Movie/video screen capture","","","","","Track/Work name","","","","The track number of this piece if part of a specific …","","","","","The version field may be used to differentiate multiple …","","","","","ABI version for libopusenc’s header. Can be used to …","Add a comment.","Add a picture from a file.","Add a picture already in memory.","Add a comment as a single tag=value string.","","","","","","","","","","","","","","","","","","","","","","","","","Ends the stream and create a new stream within the same …","","","","","","","","","","","","","Ends the stream and create a new file.","Create a new comments object.","Create a new OggOpus file.","","","Finalizes the stream.","","","","","","Write out the header now rather than wait for audio to …","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","Returns the argument unchanged.","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns a string representing the version of libopusenc …","Add/encode any number of 16-bit linear samples to the …","Add/encode any number of float samples to the stream."],"i":[14,14,14,13,14,14,14,14,19,14,10,13,14,10,14,14,14,10,19,10,14,10,19,14,19,14,0,14,10,14,14,10,14,13,13,14,13,13,14,14,14,10,10,14,0,14,14,0,10,10,13,14,14,10,14,19,19,19,21,21,13,14,14,10,10,13,14,10,13,10,14,14,0,10,14,14,14,18,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,21,0,13,14,14,14,10,19,10,13,14,0,0,14,10,14,0,14,14,14,14,14,14,14,14,14,14,14,14,14,0,10,14,14,14,18,13,19,14,14,13,14,14,14,19,13,14,14,19,14,0,2,2,2,2,13,14,13,13,14,14,13,14,2,10,13,14,16,18,21,19,2,10,13,14,16,18,21,19,16,2,10,13,14,18,19,2,10,13,14,18,19,16,2,16,2,10,16,2,16,21,19,0,16,10,13,13,14,14,16,18,21,21,19,19,2,10,13,14,16,18,21,21,21,19,19,19,2,10,13,14,16,18,21,19,21,19,2,16,21,2,10,13,14,18,19,13,14,21,19,2,10,13,14,16,18,21,19,2,10,13,14,16,18,21,19,2,10,13,14,16,18,21,19,0,16,16],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[],1],[[2,[4,[3]],[7,[[6,[5]]]]],[[8,[2]]]],[[2,[4,[9]],10,[11,[[7,[[6,[5]]]]]]],[[8,[2]]]],[[2,[4,[[12,[5]]]],10,[11,[[7,[[6,[5]]]]]]],[[8,[2]]]],[[2,[7,[[6,[5]]]]],[[8,[2]]]],[13,3],[14,3],[13,15],[13,3],[14,15],[14,3],[13,15],[14,15],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[16,[17,[2]]],8],[2,2],[10,10],[13,13],[14,14],[18,18],[19,19],[[]],[[]],[[]],[[]],[[]],[[]],[[16,[4,[9]],[17,[2]]],8],[[],2],[[[4,[9]],[17,[2]],1,20,18],[[8,[16]]]],[[],2],[[],10],[16,8],[2],[16],[[21,21],22],[[19,19],22],0,[16,8],[[10,23],24],[[13,23],24],[[13,23],24],[[14,23],24],[[14,23],24],[[16,23],24],[[18,23],24],[[21,23],24],[[21,23],24],[[19,23],24],[[19,23],24],[[]],[[]],[[]],[[]],[[]],[[]],[19,21],[25,21],[[]],[[]],[1,19],[[],19],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[26],[26],[2,27],[16,28],[21,[[11,[29]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[],30],[[],30],[[],30],[[],30],[[],31],[[],31],[[],31],[[],31],[[],31],[[],31],[[],31],[[],31],[[],31],[[],31],[[],31],[[],31],[[],31],[[],31],[[],31],[[],31],[[],32],[[],32],[[],32],[[],32],[[],32],[[],32],[[],32],[[],32],[[],3],[[16,[12,[33]]],8],[[16,[12,[34]]],8]],"c":[],"p":[[15,"i32"],[3,"Comments"],[3,"CStr"],[8,"AsRef"],[15,"u8"],[3,"Vec"],[8,"Into"],[6,"Result"],[3,"Path"],[4,"PictureType"],[4,"Option"],[15,"slice"],[4,"RecommendedTag"],[4,"PicardTag"],[15,"str"],[3,"Encoder"],[8,"Borrow"],[4,"MappingFamily"],[4,"OpusencError"],[15,"usize"],[4,"Error"],[15,"bool"],[3,"Formatter"],[6,"Result"],[3,"NulError"],[3,"Demand"],[3,"OggOpusComments"],[3,"OggOpusEnc"],[8,"Error"],[3,"String"],[4,"Result"],[3,"TypeId"],[15,"i16"],[15,"f32"]]},\
"opusenc_sys":{"doc":"Low-level binding for libopusenc.","t":"RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRDDDDDDRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRMGGGGMMGGGGGGGGDGGRGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLGGGGGGGGGLLLLLLLLDGFFFFFFFFFFFFFFFFFFFFFFFFFGFGFFFFFFFFFFFFFFFFGGGGFFFFFFFFFFFFFFFFFFFFGGGGLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLGGGGGGGGGGM","n":["INT16_MAX","INT16_MIN","INT32_MAX","INT32_MIN","INT8_MAX","INT8_MIN","INTPTR_MAX","INTPTR_MIN","INT_FAST16_MAX","INT_FAST16_MIN","INT_FAST32_MAX","INT_FAST32_MIN","INT_FAST8_MAX","INT_FAST8_MIN","INT_LEAST16_MAX","INT_LEAST16_MIN","INT_LEAST32_MAX","INT_LEAST32_MIN","INT_LEAST8_MAX","INT_LEAST8_MIN","OPE_ALLOC_FAIL","OPE_API_VERSION","OPE_BAD_ARG","OPE_CANNOT_OPEN","OPE_CLOSE_FAIL","OPE_GET_COMMENT_PADDING_REQUEST","OPE_GET_DECISION_DELAY_REQUEST","OPE_GET_HEADER_GAIN_REQUEST","OPE_GET_MUXING_DELAY_REQUEST","OPE_GET_NB_COUPLED_STREAMS_REQUEST","OPE_GET_NB_STREAMS_REQUEST","OPE_GET_SERIALNO_REQUEST","OPE_INTERNAL_ERROR","OPE_INVALID_ICON","OPE_INVALID_PICTURE","OPE_OK","OPE_SET_COMMENT_PADDING_REQUEST","OPE_SET_DECISION_DELAY_REQUEST","OPE_SET_HEADER_GAIN_REQUEST","OPE_SET_MUXING_DELAY_REQUEST","OPE_SET_PACKET_CALLBACK_REQUEST","OPE_SET_SERIALNO_REQUEST","OPE_TOO_LATE","OPE_UNIMPLEMENTED","OPE_WRITE_FAIL","OPUS_ALLOC_FAIL","OPUS_APPLICATION_AUDIO","OPUS_APPLICATION_RESTRICTED_LOWDELAY","OPUS_APPLICATION_VOIP","OPUS_AUTO","OPUS_BAD_ARG","OPUS_BANDWIDTH_FULLBAND","OPUS_BANDWIDTH_MEDIUMBAND","OPUS_BANDWIDTH_NARROWBAND","OPUS_BANDWIDTH_SUPERWIDEBAND","OPUS_BANDWIDTH_WIDEBAND","OPUS_BITRATE_MAX","OPUS_BUFFER_TOO_SMALL","OPUS_FRAMESIZE_100_MS","OPUS_FRAMESIZE_10_MS","OPUS_FRAMESIZE_120_MS","OPUS_FRAMESIZE_20_MS","OPUS_FRAMESIZE_2_5_MS","OPUS_FRAMESIZE_40_MS","OPUS_FRAMESIZE_5_MS","OPUS_FRAMESIZE_60_MS","OPUS_FRAMESIZE_80_MS","OPUS_FRAMESIZE_ARG","OPUS_GET_APPLICATION_REQUEST","OPUS_GET_BANDWIDTH_REQUEST","OPUS_GET_BITRATE_REQUEST","OPUS_GET_COMPLEXITY_REQUEST","OPUS_GET_DTX_REQUEST","OPUS_GET_EXPERT_FRAME_DURATION_REQUEST","OPUS_GET_FINAL_RANGE_REQUEST","OPUS_GET_FORCE_CHANNELS_REQUEST","OPUS_GET_GAIN_REQUEST","OPUS_GET_INBAND_FEC_REQUEST","OPUS_GET_IN_DTX_REQUEST","OPUS_GET_LAST_PACKET_DURATION_REQUEST","OPUS_GET_LOOKAHEAD_REQUEST","OPUS_GET_LSB_DEPTH_REQUEST","OPUS_GET_MAX_BANDWIDTH_REQUEST","OPUS_GET_PACKET_LOSS_PERC_REQUEST","OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST","OPUS_GET_PITCH_REQUEST","OPUS_GET_PREDICTION_DISABLED_REQUEST","OPUS_GET_SAMPLE_RATE_REQUEST","OPUS_GET_SIGNAL_REQUEST","OPUS_GET_VBR_CONSTRAINT_REQUEST","OPUS_GET_VBR_REQUEST","OPUS_INTERNAL_ERROR","OPUS_INVALID_PACKET","OPUS_INVALID_STATE","OPUS_OK","OPUS_RESET_STATE","OPUS_SET_APPLICATION_REQUEST","OPUS_SET_BANDWIDTH_REQUEST","OPUS_SET_BITRATE_REQUEST","OPUS_SET_COMPLEXITY_REQUEST","OPUS_SET_DTX_REQUEST","OPUS_SET_EXPERT_FRAME_DURATION_REQUEST","OPUS_SET_FORCE_CHANNELS_REQUEST","OPUS_SET_GAIN_REQUEST","OPUS_SET_INBAND_FEC_REQUEST","OPUS_SET_LSB_DEPTH_REQUEST","OPUS_SET_MAX_BANDWIDTH_REQUEST","OPUS_SET_PACKET_LOSS_PERC_REQUEST","OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST","OPUS_SET_PREDICTION_DISABLED_REQUEST","OPUS_SET_SIGNAL_REQUEST","OPUS_SET_VBR_CONSTRAINT_REQUEST","OPUS_SET_VBR_REQUEST","OPUS_SIGNAL_MUSIC","OPUS_SIGNAL_VOICE","OPUS_UNIMPLEMENTED","OggOpusComments","OggOpusEnc","OpusDecoder","OpusEncCallbacks","OpusEncoder","OpusRepacketizer","PTRDIFF_MAX","PTRDIFF_MIN","SIG_ATOMIC_MAX","SIG_ATOMIC_MIN","SIZE_MAX","UINT16_MAX","UINT32_MAX","UINT8_MAX","UINTPTR_MAX","UINT_FAST16_MAX","UINT_FAST32_MAX","UINT_FAST8_MAX","UINT_LEAST16_MAX","UINT_LEAST32_MAX","UINT_LEAST8_MAX","WINT_MAX","WINT_MIN","_ATFILE_SOURCE","_BITS_STDINT_INTN_H","_BITS_STDINT_UINTN_H","_BITS_TIME64_H","_BITS_TYPESIZES_H","_BITS_TYPES_H","_BITS_WCHAR_H","_DEFAULT_SOURCE","_FEATURES_H","_POSIX_C_SOURCE","_POSIX_SOURCE","_STDC_PREDEF_H","_STDINT_H","_SYS_CDEFS_H","__FD_SETSIZE","__GLIBC_MINOR__","__GLIBC_USE_DEPRECATED_GETS","__GLIBC_USE_DEPRECATED_SCANF","__GLIBC_USE_IEC_60559_BFP_EXT","__GLIBC_USE_IEC_60559_BFP_EXT_C2X","__GLIBC_USE_IEC_60559_EXT","__GLIBC_USE_IEC_60559_FUNCS_EXT","__GLIBC_USE_IEC_60559_FUNCS_EXT_C2X","__GLIBC_USE_IEC_60559_TYPES_EXT","__GLIBC_USE_ISOC2X","__GLIBC_USE_LIB_EXT2","__GLIBC__","__GNU_LIBRARY__","__HAVE_GENERIC_SELECTION","__INO_T_MATCHES_INO64_T","__KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64","__LDOUBLE_REDIRECTS_TO_FLOAT128_ABI","__OFF_T_MATCHES_OFF64_T","__RLIM_T_MATCHES_RLIM64_T","__STATFS_MATCHES_STATFS64","__STDC_IEC_559_COMPLEX__","__STDC_IEC_559__","__STDC_IEC_60559_BFP__","__STDC_IEC_60559_COMPLEX__","__STDC_ISO_10646__","__SYSCALL_WORDSIZE","__TIMESIZE","__USE_ATFILE","__USE_FORTIFY_LEVEL","__USE_ISOC11","__USE_ISOC95","__USE_ISOC99","__USE_MISC","__USE_POSIX","__USE_POSIX199309","__USE_POSIX199506","__USE_POSIX2","__USE_POSIX_IMPLICITLY","__USE_XOPEN2K","__USE_XOPEN2K8","__WORDSIZE","__WORDSIZE_TIME64_COMPAT32","__bindgen_padding_0","__blkcnt64_t","__blkcnt_t","__blksize_t","__caddr_t","__clang_max_align_nonce1","__clang_max_align_nonce2","__clock_t","__clockid_t","__daddr_t","__dev_t","__fsblkcnt64_t","__fsblkcnt_t","__fsfilcnt64_t","__fsfilcnt_t","__fsid_t","__fsword_t","__gid_t","__glibc_c99_flexarr_available","__id_t","__ino64_t","__ino_t","__int16_t","__int32_t","__int64_t","__int8_t","__int_least16_t","__int_least32_t","__int_least64_t","__int_least8_t","__intmax_t","__intptr_t","__key_t","__loff_t","__mode_t","__nlink_t","__off64_t","__off_t","__pid_t","__quad_t","__rlim64_t","__rlim_t","__sig_atomic_t","__socklen_t","__ssize_t","__suseconds64_t","__suseconds_t","__syscall_slong_t","__syscall_ulong_t","__time_t","__timer_t","__u_char","__u_int","__u_long","__u_quad_t","__u_short","__uid_t","__uint16_t","__uint32_t","__uint64_t","__uint8_t","__uint_least16_t","__uint_least32_t","__uint_least64_t","__uint_least8_t","__uintmax_t","__useconds_t","__val","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","close","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","int_fast16_t","int_fast32_t","int_fast64_t","int_fast8_t","int_least16_t","int_least32_t","int_least64_t","int_least8_t","intmax_t","into","into","into","into","into","into","into","into","max_align_t","ope_close_func","ope_comments_add","ope_comments_add_picture","ope_comments_add_picture_from_memory","ope_comments_add_string","ope_comments_copy","ope_comments_create","ope_comments_destroy","ope_encoder_chain_current","ope_encoder_continue_new_callbacks","ope_encoder_continue_new_file","ope_encoder_create_callbacks","ope_encoder_create_file","ope_encoder_create_pull","ope_encoder_ctl","ope_encoder_deferred_init_with_mapping","ope_encoder_destroy","ope_encoder_drain","ope_encoder_flush_header","ope_encoder_get_option","ope_encoder_get_page","ope_encoder_set_option","ope_encoder_write","ope_encoder_write_float","ope_get_abi_version","ope_get_version_string","ope_packet_func","ope_strerror","ope_write_func","opus_decode","opus_decode_float","opus_decoder_create","opus_decoder_ctl","opus_decoder_destroy","opus_decoder_get_nb_samples","opus_decoder_get_size","opus_decoder_init","opus_encode","opus_encode_float","opus_encoder_create","opus_encoder_ctl","opus_encoder_destroy","opus_encoder_get_size","opus_encoder_init","opus_get_version_string","opus_int16","opus_int32","opus_int64","opus_int8","opus_multistream_packet_pad","opus_multistream_packet_unpad","opus_packet_get_bandwidth","opus_packet_get_nb_channels","opus_packet_get_nb_frames","opus_packet_get_nb_samples","opus_packet_get_samples_per_frame","opus_packet_pad","opus_packet_parse","opus_packet_unpad","opus_pcm_soft_clip","opus_repacketizer_cat","opus_repacketizer_create","opus_repacketizer_destroy","opus_repacketizer_get_nb_frames","opus_repacketizer_get_size","opus_repacketizer_init","opus_repacketizer_out","opus_repacketizer_out_range","opus_strerror","opus_uint16","opus_uint32","opus_uint64","opus_uint8","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","uint_fast16_t","uint_fast32_t","uint_fast64_t","uint_fast8_t","uint_least16_t","uint_least32_t","uint_least64_t","uint_least8_t","uintmax_t","wchar_t","write"],"q":[[0,"opusenc_sys"]],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","libopus/libopusenc uses C macros to get options.","","libopus/libopusenc uses C macros to get options.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,6,1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,0,0,0,0,0,0,0,0,0,1,2,3,4,5,6,7,8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,0,0,0,0,0,0,0,0,0,0,6],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[1,1],[2,2],[3,3],[4,4],[5,5],[6,6],[7,7],[8,8],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,[[1,9],10],[[2,9],10],[[3,9],10],[[4,9],10],[[5,9],10],[[6,9],10],[[7,9],10],[[8,9],10],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[8,11,12],12],0,[[8,11,12],12],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],13],[[],13],[[],13],[[],13],[[],13],[[],13],[[],13],[[],13],[[],13],[[],13],[[],13],[[],13],[[],13],[[],13],[[],13],[[],13],[[],14],[[],14],[[],14],[[],14],[[],14],[[],14],[[],14],[[],14],0,0,0,0,0,0,0,0,0,0,0],"c":[],"p":[[3,"__fsid_t"],[3,"max_align_t"],[3,"OpusEncoder"],[3,"OpusDecoder"],[3,"OpusRepacketizer"],[3,"OpusEncCallbacks"],[3,"OggOpusComments"],[3,"OggOpusEnc"],[3,"Formatter"],[6,"Result"],[15,"u32"],[6,"c_int"],[4,"Result"],[3,"TypeId"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
