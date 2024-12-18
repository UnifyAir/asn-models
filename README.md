# asn-models

5g models generated from asn files.

Currently only ngap models are only generated 

The asn generator is from fork of [5g-asn1-protos](https://github.com/nplrkn/5g-asn1-protos), where slog has been replaced by tracing library.

### Todo

- [ ] Replace async trait with native async traits.  
- [ ] Generated other models
  - [ ] e1ap
  - [ ] f1ap
  - [ ] rrc
- [ ] Python Script add support for parsing random asn and correct file names.

### Done ✓

- [X] Replace slog with tracing library.
- [X] Python script for generating release ngap asn files from document TS 38.413 17.10.0.
- [X] Support for Default. Additional trait like Eq, PartialEq, Ord, PartialOrd, Hash on primitve newtypes
