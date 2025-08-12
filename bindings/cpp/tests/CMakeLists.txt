enable_testing()
include(CTest)

set(test_main validate-main)

set(TEST_LIST
    validate-config
    validate-password
    validate-repeats
    ${test_main}
)

foreach(TEST_NAME ${TEST_LIST})
    string(REPLACE "-" "_" src_name ${TEST_NAME})
    add_executable(${TEST_NAME} ${CMAKE_CURRENT_LIST_DIR}/${src_name}.cpp)
    target_link_libraries(${TEST_NAME} mk_pass)
    target_include_directories(${TEST_NAME} INTERFACE ${CMAKE_CURRENT_LIST_DIR}/../include)
    if("${TEST_NAME}" STREQUAL "${test_main}")
        add_test(${TEST_NAME} ${TEST_NAME} "-h")
        set_property(
            TEST ${TEST_NAME}
            PROPERTY PASS_REGULAR_EXPRESSION "^Generate a password comprehensively."
        )
    else()
        add_test(${TEST_NAME} ${TEST_NAME})
    endif()
endforeach()
