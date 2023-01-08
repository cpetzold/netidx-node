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
