#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <string>
#include <string_view>

namespace cpp2rust {

enum class RulesLanguage { kC, kCxx };

RulesLanguage GetRulesLanguage(std::string_view filename);

std::string GetRulesPreludePath(RulesLanguage language);

std::string BuildRulesPrelude(RulesLanguage language);

bool IsRulesPrelude(std::string_view path);

std::string GetRulesPreludeInclude(RulesLanguage language);

} // namespace cpp2rust
