(function() {var implementors = {};
implementors["env_logger"] = [{"text":"impl&lt;'a, T:&nbsp;UpperExp&gt; UpperExp for StyledValue&lt;'a, T&gt;","synthetic":false,"types":[]}];
implementors["itertools"] = [{"text":"impl&lt;'a, I&gt; UpperExp for Format&lt;'a, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: UpperExp,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["nalgebra"] = [{"text":"impl&lt;N, R:&nbsp;Dim, C:&nbsp;Dim, S&gt; UpperExp for Matrix&lt;N, R, C, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + UpperExp,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Storage&lt;N, R, C&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;usize, R, C&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["num_complex"] = [{"text":"impl&lt;T&gt; UpperExp for Complex&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UpperExp + Num + PartialOrd + Clone,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;UpperExp + Clone + Integer&gt; UpperExp for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
implementors["tinyvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; UpperExp for ArrayVec&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A::Item: UpperExp,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; UpperExp for TinyVec&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A::Item: UpperExp,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()