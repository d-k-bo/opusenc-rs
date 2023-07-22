// Copyright (c) 2023 d-k-bo
// SPDX-License-Identifier: BSD-3-Clause

use std::{
    ffi::{CStr, CString},
    fmt::Display,
    os::unix::prelude::OsStrExt,
    path::Path,
};

use crate::error::{CheckResult, Result};

use cstr::cstr;

/// Comments to be attached to an Ogg Opus file.
///
/// Can contain metadata in [VorbisComment](https://wiki.xiph.org/VorbisComment)
/// format and images.
///
/// # Example
///
/// ```
/// use opusenc::{Comments, PicardTag, RecommendedTag};
///
/// # fn main() -> opusenc::Result<()> {
/// let comments = Comments::create()
///     .add(RecommendedTag::Title, "The Hardest Button to Button")?
///     .add(RecommendedTag::Artist, "The White Stripes")?
///     .add(RecommendedTag::Album, "Elephant")?
///     .add(RecommendedTag::TrackNumber, "9")?
///     .add(PicardTag::AlbumArtist, "The White Stripes")?
///     .add(RecommendedTag::Isrc, "USVT10300009")?;
/// # Ok(())
/// # }
/// ```
pub struct Comments {
    ptr: *mut crate::ffi::OggOpusComments,
}

impl Drop for Comments {
    fn drop(&mut self) {
        unsafe { crate::ffi::ope_comments_destroy(self.ptr) };
    }
}

impl Comments {
    /// Create a new comments object.
    pub fn create() -> Self {
        let ptr = unsafe { crate::ffi::ope_comments_create() };
        assert!(!ptr.is_null());
        Self { ptr }
    }
}

impl Comments {
    /// Add a comment.
    ///
    /// For common tags, see [`RecommendedTag`] and [`PicardTag`].
    ///
    /// Tag names must not contain the `=` charater.
    pub fn add(&mut self, tag: impl AsRef<CStr>, val: impl Into<Vec<u8>>) -> Result<&mut Self> {
        let val = CString::new(val)?;
        unsafe { crate::ffi::ope_comments_add(self.ptr, tag.as_ref().as_ptr(), val.as_ptr()) }
            .check_result()?;

        Ok(self)
    }
    /// Add a comment as a single tag=value string.
    ///
    /// The string must contain the `=` character.
    pub fn add_string(&mut self, tag_and_val: impl Into<Vec<u8>>) -> Result<&mut Self> {
        let tag_and_val = CString::new(tag_and_val)?;
        unsafe { crate::ffi::ope_comments_add_string(self.ptr, tag_and_val.as_ptr()) }
            .check_result()?;

        Ok(self)
    }
    /// Add a picture from a file.
    pub fn add_picture(
        &mut self,
        filename: impl AsRef<Path>,
        picture_type: PictureType,
        description: Option<impl Into<Vec<u8>>>,
    ) -> Result<&mut Self> {
        let filename = CString::new(filename.as_ref().as_os_str().as_bytes())?;
        let description = description.map(CString::new).transpose()?;
        unsafe {
            crate::ffi::ope_comments_add_picture(
                self.ptr,
                filename.as_ptr(),
                picture_type as i32,
                description.map(|d| d.as_ptr()).unwrap_or(std::ptr::null()),
            )
        }
        .check_result()?;

        Ok(self)
    }
    /// Add a picture already in memory.
    pub fn add_picture_from_memory(
        &mut self,
        picture: impl AsRef<[u8]>,
        picture_type: PictureType,
        description: Option<impl Into<Vec<u8>>>,
    ) -> Result<&mut Self> {
        let picture = picture.as_ref();
        let description = description.map(CString::new).transpose()?;
        unsafe {
            crate::ffi::ope_comments_add_picture_from_memory(
                self.ptr,
                picture.as_ptr() as *const i8,
                picture.len(),
                picture_type as i32,
                description.map(|d| d.as_ptr()).unwrap_or(std::ptr::null()),
            )
        }
        .check_result()?;

        Ok(self)
    }
}

impl Comments {
    pub fn ptr(&self) -> *mut crate::ffi::OggOpusComments {
        self.ptr
    }
}

impl Clone for Comments {
    fn clone(&self) -> Self {
        let ptr = unsafe { crate::ffi::ope_comments_copy(self.ptr) };
        assert!(!ptr.is_null());
        Self { ptr }
    }
}

impl Default for Comments {
    fn default() -> Self {
        Comments::create()
    }
}

/// Type of an attached picture.
#[derive(Copy, Clone, Debug, Default)]
#[repr(i32)]
pub enum PictureType {
    /// Other
    Other = 0,
    /// 32x32 pixel 'file icon' (PNG only)
    FileIcon32 = 1,
    /// Other file icon
    OtherFileIcon = 2,
    /// Cover (front)
    #[default]
    FrontCover = 3,
    /// Cover (back)
    BackCover = 4,
    /// Leaflet page
    LeafletPage = 5,
    /// Media (e.g., label side of a CD)
    Media = 6,
    /// Lead artist/lead performer/soloist
    LeadArtist = 7,
    /// Artist/performer
    Artist = 8,
    /// Conductor
    Conductor = 9,
    /// Band/Orchestra
    BandOrchestra = 10,
    /// Composer
    Composer = 11,
    /// Lyricist/text writer
    Lyricist = 12,
    /// Recording location
    Location = 13,
    /// During recording
    DuringRecording = 14,
    /// During performance
    DuringPerformance = 15,
    /// Movie/video screen capture
    ScreenCapture = 16,
    /// A bright colored fish
    BrightColoredFish = 17,
    /// Illustration
    Illustration = 18,
    /// Band/artist logotype
    ArtistLogotype = 19,
    /// Publisher/studio logotype
    PublisherLogotype = 20,
}

macro_rules! tag_names {
    {
        $( #[$enum_attr:meta] )*
        $vis:vis enum $enum_name:ident {
            $( $(#[$attr:meta] )*
            $variant:ident = $val:literal ),* $(,)?
        }
    } => {
        $(#[$enum_attr])*
        $vis enum $enum_name {
            $(
                $( #[$attr] )*
                $variant,
            )*
        }
        impl $enum_name {
            $vis fn as_str(&self) -> &'static str {
                match self {
                    $(
                        $enum_name::$variant => $val,
                    )*
                }
            }
            $vis fn as_cstr(&self) -> &'static CStr {
                match self {
                    $(
                        $enum_name::$variant => cstr!($val),
                    )*
                }
            }
        }
        impl AsRef<str> for $enum_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
        impl AsRef<CStr> for $enum_name {
            fn as_ref(&self) -> &CStr {
                self.as_cstr()
            }
        }
        impl Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

tag_names! {
    /// Tags recommended by the Ogg Vorbis specification.
    ///
    /// <https://xiph.org/vorbis/doc/v-comment.html>
    #[derive(Copy, Clone, Debug)]
    #[non_exhaustive]
    pub enum RecommendedTag {
        /// Track/Work name
        Title = "TITLE",
        /// The version field may be used to differentiate multiple versions of the same track title in a single collection. (e.g. remix info)
        Version = "VERSION",
        /// The collection name to which this track belongs
        Album = "ALBUM",
        /// The track number of this piece if part of a specific larger collection or album
        TrackNumber = "TRACKNUMBER",
        /// The artist generally considered responsible for the work. In popular music this is usually the performing band or singer. For classical music it would be the composer. For an audio book it would be the author of the original text.
        Artist = "ARTIST",
        /// The artist(s) who performed the work. In classical music this would be the conductor, orchestra, soloists. In an audio book it would be the actor who did the reading. In popular music this is typically the same as the ARTIST and is omitted.
        Performer = "PERFORMER",
        /// Copyright attribution, e.g., '2001 Nobody's Band' or '1999 Jack Moffitt'
        Copyright = "COPYRIGHT",
        /// License information, for example, 'All Rights Reserved', 'Any Use Permitted', a URL to a license such as a Creative Commons license (e.g. "creativecommons.org/licenses/by/4.0/"), or similar.
        License = "LICENSE",
        /// Name of the organization producing the track (i.e. the 'record label')
        Organization = "ORGANIZATION",
        /// A short text description of the contents
        Description = "DESCRIPTION",
        /// A short text indication of music genre
        Genre = "GENRE",
        /// Date the track was recorded
        Date = "DATE",
        /// Location where track was recorded
        Location = "LOCATION",
        /// Contact information for the creators or distributors of the track. This could be a URL, an email address, the physical address of the producing label.
        Contact = "CONTACT",
        /// ISRC number for the track; see [the ISRC intro page](https://isrc.ifpi.org/) for more information on ISRC numbers.
        Isrc = "ISRC",
    }
}

tag_names! {
    /// Common tags used by Musicbrainz Picard.
    ///
    /// <https://picard-docs.musicbrainz.org/en/appendices/tag_mapping.html>
    ///
    /// <https://picard-docs.musicbrainz.org/downloads/MusicBrainz_Picard_Tag_Map.html>
    #[derive(Copy, Clone, Debug)]
    #[non_exhaustive]
    pub enum PicardTag {
        AcoustID = "ACOUSTID_ID",
        AcoustIDFingerprint = "ACOUSTID_FINGERPRINT",
        Album = "ALBUM",
        AlbumArtist = "ALBUMARTIST",
        AlbumArtistSortOrder = "ALBUMARTISTSORT",
        AlbumSortOrder = "ALBUMSORT",
        Arranger = "ARRANGER",
        Artist = "ARTIST",
        ArtistSortOrder = "ARTISTSORT",
        Artists = "ARTISTS",
        ASIN = "ASIN",
        Barcode = "BARCODE",
        BPM = "BPM",
        CatalogNumber = "CATALOGNUMBER",
        Comment = "COMMENT",
        Compilation = "COMPILATION",
        Composer = "COMPOSER",
        ComposerSortOrder = "COMPOSERSORT",
        Conductor = "CONDUCTOR",
        Copyright = "COPYRIGHT",
        Director = "DIRECTOR",
        DiscNumber = "DISCNUMBER",
        DiscSubtitle = "DISCSUBTITLE",
        EncodedBy = "ENCODEDBY",
        EncoderSettings = "ENCODERSETTINGS",
        Engineer = "ENGINEER",
        Genre = "GENRE",
        Grouping = "GROUPING",
        InitialKey = "KEY",
        Isrc = "ISRC",
        Language = "LANGUAGE",
        License = "LICENSE",
        Lyricist = "LYRICIST",
        Lyrics = "LYRICS",
        Media = "MEDIA",
        MixDJ = "DJMIXER",
        Mixer = "MIXER",
        Mood = "MOOD",
        Movement = "MOVEMENTNAME",
        MovementCount = "MOVEMENTTOTAL",
        MovementNumber = "MOVEMENT",
        MusicBrainzArtistID = "MUSICBRAINZ_ARTISTID",
        MusicBrainzDiscID = "MUSICBRAINZ_DISCID",
        MusicBrainzOriginalArtistID = "MUSICBRAINZ_ORIGINALARTISTID",
        MusicBrainzOriginalReleaseID = "MUSICBRAINZ_ORIGINALALBUMID",
        MusicBrainzRecordingID = "MUSICBRAINZ_TRACKID",
        MusicBrainzReleaseArtistID = "MUSICBRAINZ_ALBUMARTISTID",
        MusicBrainzReleaseGroupID = "MUSICBRAINZ_RELEASEGROUPID",
        MusicBrainzReleaseID = "MUSICBRAINZ_ALBUMID",
        MusicBrainzTrackID = "MUSICBRAINZ_RELEASETRACKID",
        MusicBrainzTRMID = "MUSICBRAINZ_TRMID",
        MusicBrainzWorkID = "MUSICBRAINZ_WORKID",
        MusicIPFingerprint = "FINGERPRINT",
        MusicIPPUID = "MUSICIP_PUID",
        OriginalFilename = "ORIGINALFILENAME",
        OriginalReleaseDate = "ORIGINALDATE",
        OriginalReleaseYear = "ORIGINALYEAR",
        Performer = "PERFORMER",
        Producer = "PRODUCER",
        Rating = "RATING",
        RecordLabel = "LABEL",
        ReleaseCountry = "RELEASECOUNTRY",
        ReleaseDate = "DATE",
        ReleaseStatus = "RELEASESTATUS",
        ReleaseType = "RELEASETYPE",
        Remixer = "REMIXER",
        ReplayGainAlbumGain = "REPLAYGAIN_ALBUM_GAIN",
        ReplayGainAlbumPeak = "REPLAYGAIN_ALBUM_PEAK",
        ReplayGainAlbumRange = "REPLAYGAIN_ALBUM_RANGE",
        ReplayGainReferenceLoudness = "REPLAYGAIN_REFERENCE_LOUDNESS",
        ReplayGainTrackGain = "REPLAYGAIN_TRACK_GAIN",
        ReplayGainTrackPeak = "REPLAYGAIN_TRACK_PEAK",
        ReplayGainTrackRange = "REPLAYGAIN_TRACK_RANGE",
        Script = "SCRIPT",
        ShowWorkMovement = "SHOWMOVEMENT",
        Subtitle = "SUBTITLE",
        TotalDiscs = "TOTALDISCS",
        TotalTracks = "TOTALTRACKS",
        TrackNumber = "TRACKNUMBER",
        TrackTitle = "TITLE",
        TrackTitleSortOrder = "TITLESORT",
        Website = "WEBSITE",
        WorkTitle = "WORK",
        Writer = "WRITER",
    }
}
