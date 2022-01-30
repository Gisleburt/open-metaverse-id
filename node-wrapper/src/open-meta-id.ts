import ffi from 'ffi-napi';

type Lib = {
  create_root_identity: () => ffi;
  free_root_identity: () => void;
}

class OpenMetaId {
  private lib: Lib

  constructor() {
    this.lib = ffi.Library('../rust-lib/target/debug/libopen_meta_id', {
      create_root_identity: ['char *', []],
      free_root_identity: ['void', ['char *']],
    });
  }

  createRootIdentity() {
      const songPtr = this.lib.create_root_identity();
      try {
        return songPtr.readCString();
      } finally {
        this.lib.free_root_identity(songPtr);
      }
  }
}


export default OpenMetaId;
