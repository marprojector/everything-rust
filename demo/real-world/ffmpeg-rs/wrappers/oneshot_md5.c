/* One-shot entry point over libavutil's MD5 context API: init, update in `chunk`-byte pieces
 * (one piece when `chunk` <= 0), final. Part of everything-rust's ffmpeg
 * case study; LGPL-2.1-or-later like the libavutil code it calls (see ../LICENSE). */
#include <stdint.h>
#include <stddef.h>

#include "libavutil/md5.h"
#include "libavutil/mem.h"

int oneshot_md5(uint8_t *dst, int dst_size, const uint8_t *src, size_t len, int chunk)
{
    struct AVMD5 *ctx;
    size_t done = 0;
    if (dst_size < 16)
        return -1;
    ctx = av_md5_alloc();
    if (!ctx)
        return -2;
    av_md5_init(ctx);
    do {
        size_t n = chunk > 0 && (size_t)chunk < len - done ? (size_t)chunk : len - done;
        av_md5_update(ctx, src + done, n);
        done += n;
    } while (done < len);
    av_md5_final(ctx, dst);
    av_free(ctx);
    return 16;
}
