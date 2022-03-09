//TIP: Import `Segmenter` trait.
use crate::segmenter::Segmenter;

//TIP: Make a small documentation of the specialized Segmenter like below.
/// <Script/Language> specialized [`Segmenter`].
///
/// This Segmenter uses [`<UsedLibraryToSegment>`] internally to segment the provided text.
/// <OptionalAdditionnalExplanations>
//
//TIP: Name the Segmenter with its purpose and not its internal behavior:
//     prefer JapaneseSegmenter (based on the Language) instead of LinderaSegmenter (based on the used Library).
//     Same for the filename, prefer `japanese.rs` instead of `lindera.rs`.
pub struct DummySegmenter;

//TIP: All specialized segmenters only need to implement the method `segment_str` of the `Segmenter` trait.
impl Segmenter for DummySegmenter {
    fn segment_str<'a>(&self, to_segment: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        //TIP: Create the iterator that will segment the provided text,
        //     Here it returns the complete text without segmenting it.
        let segment_iterator = Some(to_segment).into_iter();

        //TIP: Return the created iterator wrapping it in a Box.
        Box::new(segment_iterator)
    }
}

//TIP: Some segmentation Libraries need to initialize a instance of the Segmenter.
//     This initialization could be time-consuming and shouldn't be done at each call of `segment_str`.
//     In this case, you may want to store the initialized instance in a lazy static like below and call it in `segment_str`.
//     Otherwise, just remove below lines.
//
//TIP: Put this import at the top of the file.
// use once_cell::sync::Lazy;
//
// static LIBRARY_SEGMENTER: Lazy<LibrarySegmenter> = Lazy::new(|| LibrarySegmenter::new());

//TIP: For some reasons, like if a complete segmentation algorithm has to be implemented,
//	   you may want to implement your own iterator.
//     Otherwise, just remove below lines.
//     In the case you would implement a complete algorithm, please try to implement it lazily in the iterator.
//
// pub struct DummySegmenterIterator<'a> {
// 	to_segment: &'a str,
// };
//
// impl<'a> Iterator for DummySegmenterIterator<'a> {
//     type Item = &'a str;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

//TIP: Publish the newly implemented Segmenter:
//	   - import module by adding `mod dummy;` (filename) in `segmenter/mod.rs`
//	   - publish Segmenter by adding `pub use dummy::DummySegmenter;` in `segmenter/mod.rs`
//     - running `cargo doc --open` you should see your Segmenter in the segmenter module

//TODO @many: documents how to test the segmenter in the next PR

//TIP: Include the newly implemented Segmenter in the tokenization pipeline:
//	   - assign Segmenter to a Script and a Language by adding it in `SEGMENTERS` in `segmenter/mod.rs`
//	   - check if it didn't break any test or benhchmark
//     Your Segmenter will now be used on texts of the assigned Script and Language. Thank you for your contribution, and congratulation! 🎉
