/**
 ******************************************************************************
 * Xenia : Xbox 360 Emulator Research Project                                 *
 ******************************************************************************
 * Copyright 2022 Ben Vanik. All rights reserved.                             *
 * Released under the BSD license - see LICENSE in the root for more details. *
 ******************************************************************************
 */

#include "xenia/base/logging.h"
#include "xenia/kernel/kernel_state.h"
#include "xenia/kernel/util/shim_utils.h"
#include "xenia/kernel/xam/xam_private.h"
#include "xenia/kernel/xboxkrnl/xboxkrnl_video.h"
#include "xenia/xbox.h"

namespace xe {
namespace kernel {
namespace xam {

void XGetVideoMode_entry(pointer_t<X_VIDEO_MODE> video_mode) {
  // TODO(benvanik): actually check to see if these are the same.
  xboxkrnl::VdQueryVideoMode(std::move(video_mode));
}
DECLARE_XAM_EXPORT1(XGetVideoMode, kVideo, ExportTag::kSketchy);

dword_result_t XGetVideoCapabilities_entry() { return 0; }
DECLARE_XAM_EXPORT1(XGetVideoCapabilities, kVideo, kStub);

}  // namespace xam
}  // namespace kernel
}  // namespace xe

DECLARE_XAM_EMPTY_REGISTER_EXPORTS(Video);
