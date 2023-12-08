add_executable (aoc-08 "aoc-08.cpp")
target_link_libraries(aoc-08 AoC)
target_include_directories(aoc-08 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

