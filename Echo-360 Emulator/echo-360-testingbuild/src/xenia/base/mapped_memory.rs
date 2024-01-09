/**
 ******************************************************************************
 * Xenia : Xbox 360 Emulator Research Project                                 *
 ******************************************************************************
 * Copyright 2020 Ben Vanik. All rights reserved.                             *
 * Released under the BSD license - see LICENSE in the root for more details. *
 ******************************************************************************
 */

#ifndef XENIA_BASE_MAPPED_MEMORY_H_
#define XENIA_BASE_MAPPED_MEMORY_H_

#include <filesystem>
#include <memory>
#include <string_view>

#include "xenia/base/platform.h"

namespace xe {

class MappedMemory {
 public:
  enum class Mode {
    kRead,
    kReadWrite,
  };

  static std::unique_ptr<MappedMemory> Open(const std::filesystem::path& path,
                                            Mode mode, size_t offset = 0,
                                            size_t length = 0);
#if XE_PLATFORM_ANDROID
  static std::unique_ptr<MappedMemory> OpenForAndroidContentUri(
      const std::string_view uri, Mode mode, size_t offset = 0,
      size_t length = 0);
#endif  // XE_PLATFORM_ANDROID

  MappedMemory() : data_(nullptr), size_(0) {}
  MappedMemory(void* data, size_t size) : data_(data), size_(size) {}
  MappedMemory(const MappedMemory& mapped_memory) = delete;
  MappedMemory& operator=(const MappedMemory& mapped_memory) = delete;
  MappedMemory(MappedMemory&& mapped_memory) = delete;
  MappedMemory& operator=(MappedMemory&& mapped_memory) = delete;
  virtual ~MappedMemory() = default;

  // The mapping is still backed by the object the slice was created from, a
  // slice is not owning.
  std::unique_ptr<MappedMemory> Slice(size_t offset, size_t length) {
    return std::unique_ptr<MappedMemory>(
        new MappedMemory(data() + offset, length));
  }

  uint8_t* data() const { return reinterpret_cast<uint8_t*>(data_); }
  size_t size() const { return size_; }

  // Close, and optionally truncate file to size
  virtual void Close(uint64_t truncate_size = 0) {}
  virtual void Flush() {}

  // Changes the offset inside the file. This will update data() and size()!
  virtual bool Remap(size_t offset, size_t length) { return false; }

 protected:
  void* data_;
  size_t size_;
};

class ChunkedMappedMemoryWriter {
 public:
  virtual ~ChunkedMappedMemoryWriter() = default;

  static std::unique_ptr<ChunkedMappedMemoryWriter> Open(
      const std::filesystem::path& path, size_t chunk_size,
      bool low_address_space = false);

  virtual uint8_t* Allocate(size_t length) = 0;
  virtual void Flush() = 0;
  virtual void FlushNew() = 0;

 protected:
  ChunkedMappedMemoryWriter(const std::filesystem::path& path,
                            size_t chunk_size, bool low_address_space)
      : path_(path),
        chunk_size_(chunk_size),
        low_address_space_(low_address_space) {}

  std::filesystem::path path_;
  size_t chunk_size_;
  bool low_address_space_;
};

}  // namespace xe

#endif  // XENIA_BASE_MAPPED_MEMORY_H_
