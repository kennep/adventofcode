add_executable (aoc-07 "aoc-07.cpp")
target_link_libraries(aoc-07 AoC)
target_include_directories(aoc-07 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

