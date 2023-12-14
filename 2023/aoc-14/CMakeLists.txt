add_executable (aoc-14 "aoc-14.cpp")
target_link_libraries(aoc-14 AoC)
target_include_directories(aoc-14 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

