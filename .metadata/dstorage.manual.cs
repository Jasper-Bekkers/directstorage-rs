using System;

namespace Microsoft.Direct3D.DirectStorage
{
    // Workaround until the bitfields can be handled properly
    public struct DSTORAGE_REQUEST_OPTIONS
    {
        public UInt64 Upper;
        public UInt64 Lower;
    }
}