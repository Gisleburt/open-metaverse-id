import { Library } from 'ffi-napi';
import path from 'path';

class OpenMetaId {
  private lib;

  constructor() {
    const dylib = path.join(__dirname, 'open_meta_id.dylib');

    let temp = Library(dylib, {
      create_root_identity: ['char *', []],
      free_root_identity: ['void', ['char *']],
    });

    console.log(dylib, temp);

    this.lib = temp;
  }

  createRootIdentity(): string {
    console.log(this.lib)
      const identityPointer = this.lib.create_root_identity();
      try {
        return identityPointer.readCString();
      } finally {
        this.lib.free_root_identity(identityPointer);
      }
  }
}

export default OpenMetaId;
