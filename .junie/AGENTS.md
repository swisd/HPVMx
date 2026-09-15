# Junie Agent Guidelines

## 1. Critical Performance Guardrails (Large File Handling)
To prevent infinite "thinking" loops and context saturation, you must adhere to these strict execution rules:
* **Max Read Limit**: NEVER read more than 100 consecutive lines of code from any file in a single step.
* **Prohibited Behavior**: Do NOT dump full source files into the context window.
* **Targeted Lookups**: Always use localized line-range reads or grep search tools instead of continuous scrolling.
* **Pre-Analysis Requirement**: Always read and consult `.junie/map.json` before attempting to inspect any source code.

## 2. Dynamic map.json Maintenance Protocol
You are required to maintain a standalone application structure database located at `.junie/map.json`. You must dynamically expand and update this file as you complete tasks.

### Step-by-Step Update Loop:
1. **Read & Locate**: At the start of a task, inspect `.junie/map.json` to identify file purposes, key line regions, and structural objects.
2. **Surgical Inspection**: Query only the exact line ranges needed to perform your work.
3. **Capture & Map**: If you discover unmapped structural entities (such as structs, classes, functions, or key modules) during your work, capture their signatures.
4. **Write Back**: Before declaring a task complete, modify `.junie/map.json` to insert or update the schemas of the files you touched. Maintain the established JSON format flawlessly.


**Do not make entries for ignored files, with the exception of boot.vhd**
### Target JSON Schema Blueprint for updates:
```json
{
  "files": {
    "relative/path/to/file.ext": {
      "path": "relative/path/to/file.ext",
      "purpose": "Brief description of file responsibility.",
      "key_regions": [
        {
          "lines": "start-end",
          "description": "What this line range handles"
        }
      ],
      "objects": {
        "ObjectName": {
          "type": "type definition (e.g., pub struct, class, interface)",
          "args": {
            "fieldName": "dataType"
          },
          "returns": "dataType (if function/method)"
        }
      },
      "dependencies": [
        "relative/path/to/dependent_file.ext"
      ]
    }
  }
}
```