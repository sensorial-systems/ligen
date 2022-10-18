# Auto-generated by ligen-cmake {generator_version}

CMAKE_MINIMUM_REQUIRED(VERSION 3.0)
PROJECT("{project_name}")

IF(TARGET ${{PROJECT_NAME}})
RETURN()
ENDIF()

# INTERFACE is used to create a header-only library.
ADD_LIBRARY(${{PROJECT_NAME}} INTERFACE)
TARGET_INCLUDE_DIRECTORIES(${{PROJECT_NAME}} INTERFACE include)

# Set ligen root directory where we can find the binary libraries.
SET(LIGEN_DIR ${{CMAKE_CURRENT_SOURCE_DIR}}/../..)

IF(UNIX)
ELSEIF(WIN32)
    SET(DYLIB_NAME ffi_${{PROJECT_NAME}}.dll)
    SET(DYLIB_PATH ${{LIGEN_DIR}}/libraries/${{PROJECT_NAME}}/${{DYLIB_NAME}})
    SET(STLIB_PATH ${{DYLIB_PATH}}.lib)
    TARGET_LINK_LIBRARIES(${{PROJECT_NAME}} INTERFACE ${{STLIB_PATH}})
ENDIF()

# Detecting build type.
SET(BUILD_TYPE $<IF:$<OR:$<CONFIG:Debug>,$<CONFIG:RelWithDebInfo>>,Debug,Release>)

# Setting dynamic library output path.
SET(DYLIB_OUTPUT ${{CMAKE_BINARY_DIR}}/${{BUILD_TYPE}}/${{DYLIB_NAME}})

# Dynamic library copy operation.
ADD_CUSTOM_COMMAND(OUTPUT ${{DYLIB_OUTPUT}} COMMAND ${{CMAKE_COMMAND}} -E copy_if_different ${{DYLIB_PATH}} ${{DYLIB_OUTPUT}})
ADD_CUSTOM_TARGET(${{PROJECT_NAME}}_DYLIB DEPENDS ${{DYLIB_OUTPUT}})
ADD_DEPENDENCIES(${{PROJECT_NAME}} ${{PROJECT_NAME}}_DYLIB)
