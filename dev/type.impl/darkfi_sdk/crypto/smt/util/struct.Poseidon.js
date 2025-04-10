(function() {
    var type_impls = Object.fromEntries([["darkfi_sdk",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Poseidon%3CF,+L%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/darkfi_sdk/crypto/smt/util.rs.html#51\">Source</a><a href=\"#impl-Clone-for-Poseidon%3CF,+L%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + WithSmallOrderMulGroup&lt;3&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a>, const L: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"darkfi_sdk/crypto/smt/util/struct.Poseidon.html\" title=\"struct darkfi_sdk::crypto::smt::util::Poseidon\">Poseidon</a>&lt;F, L&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/darkfi_sdk/crypto/smt/util.rs.html#51\">Source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"darkfi_sdk/crypto/smt/util/struct.Poseidon.html\" title=\"struct darkfi_sdk::crypto::smt::util::Poseidon\">Poseidon</a>&lt;F, L&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174\">Source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","darkfi_sdk::crypto::smt::PoseidonFp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Poseidon%3CF,+L%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/darkfi_sdk/crypto/smt/util.rs.html#51\">Source</a><a href=\"#impl-Debug-for-Poseidon%3CF,+L%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + WithSmallOrderMulGroup&lt;3&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a>, const L: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"darkfi_sdk/crypto/smt/util/struct.Poseidon.html\" title=\"struct darkfi_sdk::crypto::smt::util::Poseidon\">Poseidon</a>&lt;F, L&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/darkfi_sdk/crypto/smt/util.rs.html#51\">Source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","darkfi_sdk::crypto::smt::PoseidonFp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-Poseidon%3CF,+L%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/darkfi_sdk/crypto/smt/util.rs.html#60-64\">Source</a><a href=\"#impl-Default-for-Poseidon%3CF,+L%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F: WithSmallOrderMulGroup&lt;3&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a>, const L: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"darkfi_sdk/crypto/smt/util/struct.Poseidon.html\" title=\"struct darkfi_sdk::crypto::smt::util::Poseidon\">Poseidon</a>&lt;F, L&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/darkfi_sdk/crypto/smt/util.rs.html#61-63\">Source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; Self</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","darkfi_sdk::crypto::smt::PoseidonFp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-FieldHasher%3CF,+L%3E-for-Poseidon%3CF,+L%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/darkfi_sdk/crypto/smt/util.rs.html#66-77\">Source</a><a href=\"#impl-FieldHasher%3CF,+L%3E-for-Poseidon%3CF,+L%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F: WithSmallOrderMulGroup&lt;3&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a>, const L: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"darkfi_sdk/crypto/smt/util/trait.FieldHasher.html\" title=\"trait darkfi_sdk::crypto::smt::util::FieldHasher\">FieldHasher</a>&lt;F, L&gt; for <a class=\"struct\" href=\"darkfi_sdk/crypto/smt/util/struct.Poseidon.html\" title=\"struct darkfi_sdk::crypto::smt::util::Poseidon\">Poseidon</a>&lt;F, L&gt;<div class=\"where\">where\n    P128Pow5T3: Spec&lt;F, 3, 2&gt;,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.hash\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/darkfi_sdk/crypto/smt/util.rs.html#70-72\">Source</a><a href=\"#method.hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"darkfi_sdk/crypto/smt/util/trait.FieldHasher.html#tymethod.hash\" class=\"fn\">hash</a>(&amp;self, inputs: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[F; L]</a>) -&gt; F</h4></section><section id=\"method.hasher\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/darkfi_sdk/crypto/smt/util.rs.html#74-76\">Source</a><a href=\"#method.hasher\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"darkfi_sdk/crypto/smt/util/trait.FieldHasher.html#tymethod.hasher\" class=\"fn\">hasher</a>() -&gt; Self</h4></section></div></details>","FieldHasher<F, L>","darkfi_sdk::crypto::smt::PoseidonFp"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Poseidon%3CF,+L%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/darkfi_sdk/crypto/smt/util.rs.html#54-58\">Source</a><a href=\"#impl-Poseidon%3CF,+L%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F: WithSmallOrderMulGroup&lt;3&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a>, const L: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"struct\" href=\"darkfi_sdk/crypto/smt/util/struct.Poseidon.html\" title=\"struct darkfi_sdk::crypto::smt::util::Poseidon\">Poseidon</a>&lt;F, L&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/darkfi_sdk/crypto/smt/util.rs.html#55-57\">Source</a><h4 class=\"code-header\">pub fn <a href=\"darkfi_sdk/crypto/smt/util/struct.Poseidon.html#tymethod.new\" class=\"fn\">new</a>() -&gt; Self</h4></section></div></details>",0,"darkfi_sdk::crypto::smt::PoseidonFp"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[9514]}