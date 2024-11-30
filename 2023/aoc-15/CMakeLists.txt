add_executable (aoc-15 "aoc-15.cpp")
target_link_libraries(aoc-15 AoC)
target_include_directories(aoc-15 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

