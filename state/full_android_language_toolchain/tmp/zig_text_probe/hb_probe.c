#include <stdio.h>
#include <hb.h>
#include <hb-ft.h>
#include <ft2build.h>
#include FT_FREETYPE_H

int main(void) {
    hb_buffer_t *buf = hb_buffer_create();
    hb_buffer_add_utf8(buf, "Braxon text forge", -1, 0, -1);
    hb_buffer_guess_segment_properties(buf);
    printf("harfbuzz buffer ok, glyphs=%u\n", hb_buffer_get_length(buf));
    hb_buffer_destroy(buf);

    FT_Library ft;
    if (FT_Init_FreeType(&ft) == 0) {
        printf("freetype init ok\n");
        FT_Done_FreeType(ft);
    }
    return 0;
}
