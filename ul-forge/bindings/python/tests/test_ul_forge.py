"""Tests for ul-forge Python bindings (PyO3/maturin).

Run with: maturin develop && pytest tests/
"""
import pytest


def _import_ul_forge():
    """Try to import the binding module; skip if not built."""
    try:
        import ul_forge  # noqa: F811
        return ul_forge
    except ImportError:
        pytest.skip("ul_forge not built — run `maturin develop` first")


def test_parse_valid():
    ul = _import_ul_forge()
    result = ul.parse("○{●}")
    assert isinstance(result, dict)
    assert "nodes" in result
    assert "edges" in result
    assert len(result["nodes"]) > 0


def test_parse_invalid_returns_error():
    ul = _import_ul_forge()
    with pytest.raises((ValueError, Exception)):
        ul.parse("○{")


def test_render_produces_svg():
    ul = _import_ul_forge()
    gir = ul.parse("●")
    svg = ul.render(gir)
    assert isinstance(svg, str)
    assert "<svg" in svg


def test_validate_valid_gir():
    ul = _import_ul_forge()
    gir = ul.parse("○{●}")
    result = ul.validate(gir)
    assert isinstance(result, dict)
    assert result["valid"] is True


def test_deparse_roundtrip():
    ul = _import_ul_forge()
    gir = ul.parse("●")
    text = ul.deparse(gir)
    assert isinstance(text, str)
    assert len(text) > 0
