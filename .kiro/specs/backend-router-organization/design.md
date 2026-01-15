# Design Document: Backend Router Organization

## Overview

This design reorganizes the FastAPI backend router structure into logical categories to improve maintainability, discoverability, and code organization. The current flat router structure will be transformed into a hierarchical category-based system while maintaining full backward compatibility.

## Architecture

### Current State
```
Backend/routers/
├── erme203.py      # Data extraction tool
├── misc.py         # General utilities
├── password.py     # Password generation
├── regex.py        # Regular expression testing
├── timestamp.py    # Time conversion
├── diff.py         # Text comparison
└── text.py         # Text processing
```

### Target State
```
Backend/routers/
├── __init__.py                    # Router registration system
├── tools/
│   ├── __init__.py
│   ├── password.py               # Password generation
│   ├── regex.py                  # Regular expression testing
│   ├── text.py                   # Text processing
│   └── diff.py                   # Text comparison
├── time/
│   ├── __init__.py
│   └── timestamp.py              # Time conversion
├── data/
│   ├── __init__.py
│   └── erme203.py                # Data extraction
├── maps/
│   ├── __init__.py
│   └── (future map-related routers)
└── misc/
    ├── __init__.py
    └── misc.py                   # General utilities
```

## Components and Interfaces

### 1. Router Discovery System

**RouterDiscovery Class**
```python
class RouterDiscovery:
    def __init__(self, base_path: str, categories: Dict[str, str])
    def discover_routers(self) -> List[APIRouter]
    def register_category_routers(self, category: str) -> List[APIRouter]
    def validate_router(self, router: APIRouter) -> bool
```

**Responsibilities:**
- Automatically discover routers in category directories
- Validate router structure and configuration
- Handle dynamic router registration

### 2. Category Configuration Manager

**CategoryConfig Class**
```python
class CategoryConfig:
    def __init__(self, config_path: str = "router_config.json")
    def load_categories(self) -> Dict[str, CategoryInfo]
    def validate_config(self) -> bool
    def get_category_mapping(self) -> Dict[str, str]
```

**Configuration Schema:**
```json
{
  "categories": {
    "tools": {
      "description": "Utility and processing tools",
      "routers": ["password", "regex", "text", "diff"]
    },
    "time": {
      "description": "Time-related functionality", 
      "routers": ["timestamp"]
    },
    "data": {
      "description": "Data extraction and processing",
      "routers": ["erme203"]
    },
    "maps": {
      "description": "Map and location services",
      "routers": []
    },
    "misc": {
      "description": "General utilities",
      "routers": ["misc"]
    }
  }
}
```

### 3. Router Registration System

**RouterRegistry Class**
```python
class RouterRegistry:
    def __init__(self, app: FastAPI, discovery: RouterDiscovery)
    def register_all_routers(self) -> None
    def register_category(self, category: str) -> None
    def get_registered_routers(self) -> List[str]
```

**Registration Flow:**
1. Load category configuration
2. Discover routers in each category directory
3. Validate router compatibility
4. Register routers with FastAPI app
5. Preserve existing prefixes and tags

## Data Models

### CategoryInfo
```python
@dataclass
class CategoryInfo:
    name: str
    description: str
    routers: List[str]
    directory_path: str
```

### RouterInfo
```python
@dataclass
class RouterInfo:
    name: str
    category: str
    file_path: str
    router_instance: APIRouter
    prefix: str
    tags: List[str]
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

Based on the prework analysis, here are the key correctness properties:

**Property 1: Category Organization Consistency**
*For any* system startup, all routers should be organized into their predefined categories according to the configuration mapping.
**Validates: Requirements 1.1, 1.2, 1.3, 1.4**

**Property 2: Directory Structure Creation**
*For any* reorganization operation, the system should create the expected category-based directory structure with all necessary subdirectories.
**Validates: Requirements 2.1, 2.2**

**Property 3: Import Statement Updates**
*For any* router file moved to a new category directory, all import statements referencing that router should be updated to reflect the new path.
**Validates: Requirements 2.3**

**Property 4: Backward Compatibility Preservation**
*For any* existing API endpoint, the URL, request/response schemas, and behavior should remain identical after reorganization.
**Validates: Requirements 1.5, 5.1, 5.2, 5.4**

**Property 5: Router Discovery Completeness**
*For any* category directory containing router files, all valid routers should be automatically discovered and registered with the FastAPI application.
**Validates: Requirements 3.1, 3.2, 3.3**

**Property 6: Metadata Preservation**
*For any* router during registration, the original prefixes, tags, and configuration should be preserved exactly.
**Validates: Requirements 3.4, 5.3**

**Property 7: Configuration Validation**
*For any* category configuration file, invalid configurations should be detected, logged, and trigger fallback to default categories.
**Validates: Requirements 4.3, 4.4**

**Property 8: Dynamic Reconfiguration**
*For any* configuration change, the new organization rules should be applied correctly without breaking existing functionality.
**Validates: Requirements 4.2**

<function_calls>
<invoke name="prework">
<parameter name="featureName">backend-router-organization

## Error Handling

### Configuration Errors
- **Invalid JSON**: Log error and use default category configuration
- **Missing Categories**: Create default categories (tools, time, data, maps, misc)
- **Invalid Router Paths**: Skip invalid routers and log warnings

### Router Discovery Errors
- **Import Failures**: Log import errors and continue with other routers
- **Invalid Router Structure**: Skip malformed routers with detailed error logging
- **Duplicate Router Names**: Use first discovered router and warn about duplicates

### File System Errors
- **Permission Issues**: Log errors and attempt to continue with available operations
- **Missing Directories**: Create missing category directories automatically
- **File Move Failures**: Log errors and maintain original file locations

### Runtime Errors
- **Registration Failures**: Log detailed error information and continue with other routers
- **Circular Dependencies**: Detect and report circular import issues
- **Memory Issues**: Implement graceful degradation for large router sets

## Testing Strategy

### Unit Testing Approach
- **Router Discovery**: Test individual router file discovery and validation
- **Configuration Loading**: Test various configuration file scenarios
- **Category Management**: Test category creation and router assignment
- **Import Updates**: Test import statement modification logic
- **Error Handling**: Test all error conditions and recovery mechanisms

### Property-Based Testing Approach
Using **pytest** with **hypothesis** library for property-based testing:

- **Minimum 100 iterations** per property test
- **Tag format**: `# Feature: backend-router-organization, Property {number}: {property_text}`
- Each property test validates universal behavior across all valid inputs

**Property Test Configuration:**
```python
from hypothesis import given, strategies as st
import pytest

@given(st.dictionaries(st.text(), st.lists(st.text())))
def test_category_organization_consistency(category_mapping):
    # Feature: backend-router-organization, Property 1: Category Organization Consistency
    # Test that routers are consistently organized according to configuration
    pass
```

**Integration Testing:**
- **End-to-End Workflow**: Test complete reorganization process
- **API Compatibility**: Verify all existing endpoints continue working
- **Performance Impact**: Ensure reorganization doesn't degrade performance
- **Rollback Scenarios**: Test ability to revert changes if needed

**Test Data Management:**
- Use temporary directories for file system tests
- Mock FastAPI applications for router registration tests
- Generate synthetic router configurations for comprehensive testing
- Test with both valid and invalid router files