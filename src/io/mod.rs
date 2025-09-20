//! Readers and writers for common bioinformatics file formats.

#[path = "bed/bed.rs"]
pub mod bed;

#[path = "bed/narrowpeak.rs"]
pub mod narrow_peak;

#[path = "bed/common.rs"]
pub mod common;

pub mod bedpe;
pub mod fasta;
pub mod fastq;
pub mod fastx;
pub mod gff;
#[cfg(feature = "phylogeny")]
pub mod newick;
