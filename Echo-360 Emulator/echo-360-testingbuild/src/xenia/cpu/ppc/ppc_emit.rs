/**
 ******************************************************************************
 * Xenia : Xbox 360 Emulator Research Project                                 *
 ******************************************************************************
 * Copyright 2013 Ben Vanik. All rights reserved.                             *
 * Released under the BSD license - see LICENSE in the root for more details. *
 ******************************************************************************
 */

#ifndef XENIA_CPU_PPC_PPC_EMIT_H_
#define XENIA_CPU_PPC_PPC_EMIT_H_

namespace xe {
namespace cpu {
namespace ppc {

void RegisterEmitCategoryAltivec();
void RegisterEmitCategoryALU();
void RegisterEmitCategoryControl();
void RegisterEmitCategoryFPU();
void RegisterEmitCategoryMemory();

}  // namespace ppc
}  // namespace cpu
}  // namespace xe

#endif  // XENIA_CPU_PPC_PPC_EMIT_H_
