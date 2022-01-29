const ffi = require('ffi-napi');

const lib = ffi.Library('../rust-lib/target/debug/libopen_meta_id', {
  create_root_identity: ['char *', []],
  free_root_identity: ['void', ['char *']],
});

function hello_world() {
  const songPtr = lib.create_root_identity();
  try {
    return songPtr.readCString();
  } finally {
    lib.free_root_identity(songPtr);
  }
}

console.log(hello_world());
