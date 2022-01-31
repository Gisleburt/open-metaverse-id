import { Library } from 'ffi-napi';

class OpenMetaId {
  private lib;

  constructor() {
    this.lib = Library('libopen_meta_id', {
      create_root_identity: ['char *', []],
      free_root_identity: ['void', ['char *']],
    });
  }

  createRootIdentity(): string {
      const identityPointer = this.lib.create_root_identity();
      try {
        return identityPointer.readCString();
      } finally {
        this.lib.free_root_identity(identityPointer);
      }
  }
}

export default OpenMetaId;
