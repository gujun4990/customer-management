# Customer Management Task 1.1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Choose the smallest viable phase-one stack and initialize a bootable CRM application skeleton.

**Architecture:** Use a Python FastAPI application with a server-rendered web entrypoint and a small package layout under `app/`. This keeps the MVP full-stack in one runtime, avoids a separate frontend build, and leaves room for later domain, persistence, and template modules.

**Tech Stack:** Python 3.10, FastAPI, Uvicorn, Jinja2, Pytest, HTTPX

---

### Task 1: Bootstrap the runnable app skeleton

**Files:**
- Create: `pyproject.toml`
- Create: `app/__init__.py`
- Create: `app/main.py`
- Create: `tests/test_app.py`

- [ ] **Step 1: Write the failing smoke test**

```python
from fastapi.testclient import TestClient

from app.main import app


def test_health_endpoint_returns_ok() -> None:
    client = TestClient(app)

    response = client.get("/health")

    assert response.status_code == 200
    assert response.json() == {"status": "ok", "app": "customer-management"}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `pytest tests/test_app.py -v`
Expected: FAIL because `app.main` does not exist yet

- [ ] **Step 3: Write minimal implementation**

```python
from fastapi import FastAPI


app = FastAPI(title="Customer Management")


@app.get("/health")
def health() -> dict[str, str]:
    return {"status": "ok", "app": "customer-management"}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `pytest tests/test_app.py -v`
Expected: PASS

- [ ] **Step 5: Confirm structure supports later tasks**

Check that the repo now has:

```text
pyproject.toml
app/
  __init__.py
  main.py
tests/
  test_app.py
```

This is sufficient for task 1.1 because it chooses the stack and creates the smallest runnable project structure without pulling task 1.2 data-model work forward.
