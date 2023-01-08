import test from "ava";

import { path } from "../";

test("path.root", (t) => {
  t.is(path.root(), "/");
});

test("path.isAbsolute", (t) => {
  t.true(path.isAbsolute("/"));
  t.true(path.isAbsolute("/foo"));
  t.true(path.isAbsolute("/foo/bar"));
  t.false(path.isAbsolute(""));
  t.false(path.isAbsolute("foo"));
  t.false(path.isAbsolute("foo/bar"));
});

test("path.isParent", (t) => {
  t.true(path.isParent("/", "/foo/bar/baz"));
  t.true(path.isParent("/foo/bar", "/foo/bar/baz"));
  t.false(path.isParent("/foo/bar", "/foo/bareth/bazeth"));
  t.true(path.isParent("/foo/bar", "/foo/bar"));
});

test("path.isImmediateParent", (t) => {
  t.false(path.isImmediateParent("/", "/foo/bar/baz"));
  t.true(path.isImmediateParent("/foo/bar", "/foo/bar/baz"));
  t.false(path.isImmediateParent("/foo/bar", "/foo/bareth/bazeth"));
  t.false(path.isImmediateParent("/foo/bar", "/foo/bar"));
  t.false(path.isImmediateParent("/", "/"));
});

test("path.stripPrefix", (t) => {
  t.is(path.stripPrefix("/", "/"), "");
  t.is(path.stripPrefix("/", "/foo"), "foo");
  t.is(path.stripPrefix("/", "foo"), null);
  t.is(path.stripPrefix("/foo", "/foo/bar"), "bar");
});
