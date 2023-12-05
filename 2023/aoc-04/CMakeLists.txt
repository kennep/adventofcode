add_executable (aoc-04 "aoc-04.cpp")
target_link_libraries(aoc-04 AoC)
target_include_directories(aoc-04 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

