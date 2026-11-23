/* One-shot entry points over libavutil's CRC API, so a single recorded call covers a table
 * lookup or a table build plus the checksum. Part of everything-rust's ffmpeg
 * case study; LGPL-2.1-or-later like the libavutil code it calls (see ../LICENSE). */
#include <stdint.h>
#include <stddef.h>

#include "libavutil/crc.h"

uint32_t oneshot_crc(int id, uint32_t crc, const uint8_t *buf, size_t len)
{
    return av_crc(av_crc_get_table(id), crc, buf, len);
}

int64_t oneshot_crc_custom(int le, int bits, uint32_t poly, int table_size, uint32_t crc,
                           const uint8_t *buf, size_t len)
{
    AVCRC table[1024];
    int ret;
    if (table_size != 257 && table_size != 1024)
        table_size = 1;
    ret = av_crc_init(table, le, bits, poly, table_size * (int)sizeof(AVCRC));
    if (ret < 0)
        return ret;
    return av_crc(table, crc, buf, len);
}
