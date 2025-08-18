<script>
	import Back from '$lib/components/Back.svelte';
</script>

<main class="column pb-32">
	<Back href="/" />
	<h1 class="text-3xl font-bold">About</h1>
	<p>
		This page provides a way to understand non-classical logics by allowing you to tinker with
		tableaux. The content (and this whole page, in fact) is modelled after <a
			href="https://www.cambridge.org/it/universitypress/subjects/philosophy/philosophy-science/introduction-non-classical-logic-if-2nd-edition"
			target="_blank"
			rel="noreferrer"
		>
			Graham Priest's <em>An Introduction to Non-Classical Logic</em>
		</a>. I started reading the book and thought that the procedure for tableaux was very
		computational, so I thought I'd write a program to make the computer do it! This turned out to
		also be a very good way to better understand the contents of the book :)
	</p>
	<p>Speaking of which...</p>

	<h2 class="mt-6 text-2xl font-bold">What are tableaux?</h2>
	<p>Tableaux are used to verify if a statement in some kind of logic holds.</p>
	<p>
		Statements are comprised of premises and a conclusion. The statement holds if the conclusion is
		true for any interpretation that makes the premises true (an interpretation being an assignment
		of truth values to a set of variables used).
	</p>
	<p>
		The tableau itself is a tree. At the root, we put a chain of premises and the negation of the
		conclusion. Then, each logic has some rules on how to expand and split each possible expression
		until you reach expressions containing just a single variable.
	</p>
	<p>
		Each branch represents a possible assignment of values and, if at any point, the branch contains
		a contradiction (such as p and ¬p) then the branch is killed. If we reach a point where every
		branch is dead, the statement holds.
	</p>
	<p>
		Essentially, tableaux are a systematic way of finding possible interpretations of an expression,
		and we find whether a statement is true by trying to find a counterexample. That is, we try to
		find an interpretation where the premises are true and the conclusion is false (that's why we
		negate the conclusion at the start!).
	</p>

	<h2 class="mt-6 text-2xl font-bold">Why and how was this made.</h2>
	<p>
		The tableaux procedure outlined above seemed computationally straightfowrard to me, and I felt
		it was beggin to be implemented with an actual computer.
	</p>
	<p>
		I did the core tableaux implementation in Rust, since it seems a nice "black box", clear inputs
		and outputs type of problem that needs correctness which is well-suited for Rust. It does use a
		lot of trees (the tableau itself is a tree, but so are expressions, which means tableaux are
		trees where every node is another tree!) and people say you can't do trees in Rust; but you can,
		and it's not that bad. The tableau itself uses a <code>Vec</code> of nodes and
		<code>NodeId</code>s as indices in the <code>Vec</code>, while the expressions use boxing.
	</p>
	<p>
		The Rust implementation seems efficient, but I'm sure it can be made even better. This could be
		useful for more complicated logics and such.
	</p>
	<p>
		The final nice thing about Rust is that you can compile it to WebAssembly, so all tableaux in
		this website run locally and efficiently on your browser!
	</p>
	<p>
		And, as the final part of the stack, this website is a SvelteKit site, I'm using Nix to package
		it and the font is Libertinus, inspired by the default option in Typst.
	</p>
	<p>
		<a href="https://github.com/odilf/tableaux">You can see the source code here</a>. Thanks for
		reading, have fun!
	</p>
</main>

<style lang="postcss">
	@reference "../../app.css";

	p {
		@apply mt-1 text-justify text-lg opacity-90;
	}
</style>
