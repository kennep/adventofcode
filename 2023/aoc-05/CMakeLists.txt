add_executable (aoc-05 "aoc-05.cpp")
target_link_libraries(aoc-05 AoC)
target_include_directories(aoc-05 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

