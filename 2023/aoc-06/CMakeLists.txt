add_executable (aoc-06 "aoc-06.cpp")
target_link_libraries(aoc-06 AoC)
target_include_directories(aoc-06 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

