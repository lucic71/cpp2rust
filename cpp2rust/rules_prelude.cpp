// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "rules_prelude.h"

#include <algorithm>
#include <cctype>
#include <filesystem>
#include <format>
#include <fstream>
#include <iterator>
#include <set>
#include <sstream>
#include <vector>

namespace fs = std::filesystem;

namespace cpp2rust {

namespace {

std::string readFile(const fs::path &path) {
  std::ifstream file(path);
  return {std::istreambuf_iterator<char>(file),
          std::istreambuf_iterator<char>()};
}

std::vector<std::string> splitLines(const std::string &text) {
  std::vector<std::string> lines;
  std::istringstream stream(text);
  for (std::string line; std::getline(stream, line);) {
    lines.push_back(std::move(line));
  }
  return lines;
}

std::string_view trimLeft(std::string_view text) {
  while (!text.empty() && std::isspace((unsigned char)text.front())) {
    text.remove_prefix(1);
  }
  return text;
}

bool isInclude(std::string_view line) {
  line = trimLeft(line);
  if (!line.starts_with('#')) {
    return false;
  }
  return trimLeft(line.substr(1)).starts_with("include");
}

std::string withoutIncludes(const std::string &text) {
  std::string out;
  for (const auto &line : splitLines(text)) {
    if (!isInclude(line)) {
      out += line;
    }
    out += '\n';
  }
  return out;
}

std::set<std::string> getRuleNames(const std::string &text) {
  std::set<std::string> names;
  auto is_word = [](char c) {
    return std::isalnum((unsigned char)c) || c == '_';
  };
  for (size_t i = 0; i < text.size();) {
    if (!is_word(text[i])) {
      ++i;
      continue;
    }
    size_t end = i;
    while (end < text.size() && is_word(text[end])) {
      ++end;
    }
    std::string_view word(text.data() + i, end - i);
    if (word.size() > 1 && (word[0] == 'f' || word[0] == 't') &&
        std::all_of(word.begin() + 1, word.end(),
                    [](char c) { return std::isdigit((unsigned char)c); })) {
      names.emplace(word);
    }
    i = end;
  }
  return names;
}

void addCxxModule(std::string &out, const fs::path &module_dir) {
  out += std::format("namespace cpp2rust_rules_{} {{\n",
                     module_dir.filename().string());
  out += withoutIncludes(readFile(module_dir / "src.cpp"));
  out += withoutIncludes(readFile(module_dir / "src.c"));
  out += "}\n";
}

void addCRules(std::string &out, const std::string &prefix,
               const std::string &text) {
  auto names = getRuleNames(text);
  for (const auto &name : names) {
    out += std::format("#define {} {}_{}\n", name, prefix, name);
  }
  out += withoutIncludes(text);
  for (const auto &name : names) {
    out += std::format("#undef {}\n", name);
  }
}

void addCModule(std::string &out, const fs::path &module_dir) {
  auto prefix = "cpp2rust_rules_" + module_dir.filename().string();
  addCRules(out, prefix, readFile(module_dir / "src.c"));
}

} // namespace

RulesLanguage GetRulesLanguage(std::string_view filename) {
  return filename.ends_with(".c") ? RulesLanguage::kC : RulesLanguage::kCxx;
}

std::string GetRulesPreludePath(RulesLanguage language) {
  return std::string(RULES_SOURCE_DIR) + (language == RulesLanguage::kC
                                              ? "/cpp2rust_rules_prelude.c"
                                              : "/cpp2rust_rules_prelude.cpp");
}

bool IsRulesPrelude(std::string_view path) {
  return path == GetRulesPreludePath(RulesLanguage::kC) ||
         path == GetRulesPreludePath(RulesLanguage::kCxx);
}

std::string GetRulesPreludeInclude(RulesLanguage language) {
  return std::format("\n#include \"{}\"\n", GetRulesPreludePath(language));
}

std::string BuildRulesPrelude(RulesLanguage language) {
  std::vector<fs::path> modules;
  for (const auto &entry : fs::directory_iterator(RULES_SOURCE_DIR)) {
    if (entry.is_directory()) {
      modules.push_back(entry.path());
    }
  }
  std::sort(modules.begin(), modules.end());

  std::string out = "#pragma GCC system_header\n";
  for (const auto &module_dir : modules) {
    if (language == RulesLanguage::kCxx) {
      addCxxModule(out, module_dir);
    } else {
      addCModule(out, module_dir);
    }
  }
  return out;
}

} // namespace cpp2rust
