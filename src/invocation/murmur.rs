// pub fn murmur(key: Vec<u8>) -> i32 {
//   let h1 = 0x01000193; // seed
//   let h1b = 0;
//   let mut k1 = 0;
//
//   let remainder = key.length & 3; // key.length % 4
//   let bytes = key.length - remainder;
//   let c1 = 0xcc9e2d51;
//   let c2 = 0x1b873593;
//
//   let mut i = 0;
//   while i < bytes {
//     // little-endian load order
//     k1 =
//         (key.readUInt8(i) & 0xff) |
//             ((key.readUInt8({
//               i += 1;
//               i
//             }) & 0xff) << 8) |
//             ((key.readUInt8({
//               i += 1;
//               i
//             }) & 0xff) << 16) |
//             ((key.readUInt8({
//               i += 1;
//               i
//             }) & 0xff) << 24);
//     i += 1;
//
//     k1 = ((((k1 & 0xffff) * c1) + ((((k1 >> > 16) * c1) & 0xffff) << 16))) & 0xffffffff;
//     // ROTL32(k1,15);
//     k1 = (k1 << 15) | (k1 >> > 17);
//     k1 = ((((k1 & 0xffff) * c2) + ((((k1 >> > 16) * c2) & 0xffff) << 16))) & 0xffffffff;
//
//     h1 ^= k1;
//     // ROTL32(h1,13);
//     h1 = (h1 << 13) | (h1 >> > 19);
//     h1b = ((((h1 & 0xffff) * 5) + ((((h1 >> > 16) * 5) & 0xffff) << 16))) & 0xffffffff;
//     h1 = (((h1b & 0xffff) + 0x6b64) + ((((h1b >> > 16) + 0xe654) & 0xffff) << 16));
//   }
//
//   // tail
//   k1 = 0;
//
//   switch(remainder)
//   {
//     case
//     3:
//         k1 ^= (key.readUInt8(i + 2) & 0xff) << 16;
//     // fallthrough
//     case
//     2: // eslint-disable-line no-fallthrough
//         k1 ^= (key.readUInt8(i + 1) & 0xff) << 8;
//     // fallthrough
//     case
//     1: // eslint-disable-line no-fallthrough
//         k1 ^= (key.readUInt8(i) & 0xff);
//
//     k1 = (((k1 & 0xffff) * c1) + ((((k1 >> > 16) * c1) & 0xffff) << 16)) & 0xffffffff;
//     // ROTL32(k1,15);
//     k1 = (k1 << 15) | (k1 >> > 17);
//     k1 = (((k1 & 0xffff) * c2) + ((((k1 >> > 16) * c2) & 0xffff) << 16)) & 0xffffffff;
//     h1 ^= k1;
//   }
//
//   // finalization
//   h1 ^= key.length;
//
//   h1 ^= h1 >> > 16;
//   h1 = (((h1 & 0xffff) * 0x85ebca6b) + ((((h1 >> > 16) * 0x85ebca6b) & 0xffff) << 16)) & 0xffffffff;
//   h1 ^= h1 >> > 13;
//   h1 = ((((h1 & 0xffff) * 0xc2b2ae35) + ((((h1 >> > 16) * 0xc2b2ae35) & 0xffff) << 16))) & 0xffffffff;
//   h1 ^= h1 >> > 16;
//
//   const result = h1 >> > 0;
//
//   // This simulates the 32 bit integer overflow to match Java implementation
//   return result | 0;
// }