add_executable (aoc-09 "aoc-09.cpp")
target_link_libraries(aoc-09 AoC)
target_include_directories(aoc-09 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

