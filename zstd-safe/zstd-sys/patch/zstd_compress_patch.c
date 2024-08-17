/*-*************************************
*  Dependencies
***************************************/
#include "common/allocations.h"  /* ZSTD_customMalloc, ZSTD_customCalloc, ZSTD_customFree */
#include "common/zstd_deps.h"  /* INT_MAX, ZSTD_memset, ZSTD_memcpy */
#include "common/mem.h"
#include "compress/zstd_compress_internal.h"

static void ZSTD_initCCtx_v2(ZSTD_CCtx* cctx, ZSTD_customMem memManager)
{
    assert(cctx != NULL);
    ZSTD_memset(cctx, 0, sizeof(*cctx));
    cctx->customMem = memManager;
    cctx->bmi2 = 0; //ZSTD_cpuSupportsBmi2();
    {   size_t const err = ZSTD_CCtx_reset(cctx, ZSTD_reset_parameters);
        assert(!ZSTD_isError(err));
        (void)err;
    }
}


ZSTD_CCtx* ZSTD_createCCtx_advanced_v2(ZSTD_customMem customMem)
{
    ZSTD_STATIC_ASSERT(zcss_init==0);
    ZSTD_STATIC_ASSERT(ZSTD_CONTENTSIZE_UNKNOWN==(0ULL - 1));
    if ((!customMem.customAlloc) ^ (!customMem.customFree)) return NULL;
    {   ZSTD_CCtx* const cctx = (ZSTD_CCtx*)ZSTD_customMalloc(sizeof(ZSTD_CCtx), customMem);
        if (!cctx) return NULL;
        ZSTD_initCCtx_v2(cctx, customMem);
        return cctx;
    }
}