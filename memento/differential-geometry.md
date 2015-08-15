---
layout: post
title: "Differential Geometry"
---

> Based on Barrett O'Neill's "Elementary Differential Geometry"


## Calculus on Euclidean Space

__Def.__ Euclidean 3-space \\( \mathbf R^3 = \lbrace (p\_1, p\_2, p\_3) :
p\_i \in \mathbf R \rbrace \\), \\( \mathbf p \in \mathbf R^3 \\) is called a
_point_ in \\( \mathbf R^3 \\).

<p><div class="tab-r">
\( x(\mathbf p) = x_1(\mathbf p) = p_1 \) <br/>
\( y(\mathbf p) = x_2(\mathbf p) = p_2 \) <br/>
\( z(\mathbf p) = x_3(\mathbf p) = p_3 \)
</div></p>

__Def.__ \\( f : \mathbf R^3 \rightarrow \mathbf R\\) is (infinitely)
differentiable / smooth / of class \\( C^\infty \\) if all of its partial
derivatives, of all orders, exist and are continuous.

Differentiation is a _local_ operation; it needs only to be defined in an open
set \\( \mathcal O \\) &mdash; if \\( \mathbf p \in \mathcal O \\), then every
point _near_ \\( \mathbf p \\) is also in \\( \mathcal O \\).

__Def.__ A _tangent vector_ \\( \mathbf v\_p \\) to \\( \mathbf R^3 \\) has a
_vector part_ \\( \mathbf v \\) and a _point of application_ \\( \mathbf p \\).

<p><div class="tab-r">
\( \mathbf v_p = \mathbf w_q \iff \mathbf v = \mathbf w \land \mathbf p = \mathbf q \)<br/>
\( \mathbf v_p \parallel \mathbf w_q \iff \mathbf v \parallel \mathbf w \)
</div></p>

__Def.__ Let \\( \mathbf p \in \mathbf R^3 \\). The set \\( T\_p(\mathbf R^3) \\)
consisting of all tangent vectors which have \\( \mathbf p \\) as their point of
application is called the _tangent space_ of \\( \mathbf R^3 \\) at \\( \mathbf p \\).

<p><div class="tab-r">
\( T_p(\mathbf R^3) = \lbrace \mathbf v_p : \mathbf v \in \mathbf R^3 \rbrace \)<br/>
\( T_p(\mathbf R^3) \neq T_q(\mathbf R^3) \) if \( \mathbf p \neq \mathbf q \)<br/>
\( \mathbf v_p + \mathbf w_p = ( \mathbf v + \mathbf w )_p \)<br/>
\( c \cdot \mathbf v_p = (c \cdot \mathbf v)_p \)<br/><span hidden>_</span>
</div></p>

\\( T\_p(\mathbf R^3) \\) is a vector space isomorphic to \\( \mathbf R^3 \\).
That is, \\( \mathbf v \rightarrow \mathbf v\_p \\) is a _linear isomorphism_, a
linear transformation that is both one-to-one and onto &mdash; in this case, it's
a translation by \\( \mathbf p \\).

__Def.__ A _vector field_ \\( V \\) on \\( \mathbf R^3 \\) is a function that
assigns to each point \\( \mathbf p \in \mathbf R^3 \\) a tangent vector
\\( V(\mathbf p) \\) to \\( \mathbf R^3 \\) at \\( \mathbf p \\), i.e.
\\( V(\mathbf p) \rightarrow \mathbf v\_p \\).

__Pointwise principle:__ if a certain operation can be performed on the values
of two functions \\( V, W : \mathbf R^3 \rightarrow \mathbf R \\) at the same
point, then that operation can be extended to the functions themselves.

<p><div class="tab-r">
\\( V(\mathbf p) + W(\mathbf p) \equiv (V + W)(\mathbf p) \\)<br/>
\\( V(\mathbf p) \cdot W(\mathbf p) \equiv (V \cdot W)(\mathbf p) \\)
</p></div>

__Def.__ Let \\( U\_1 \\), \\( U\_2 \\), \\( U\_3 \\) be vector fields on
\\( \mathbf R^3 \\) such that

<p><div class="tab-r">
\\( U\_1(\mathbf p) = (1, 0, 0)\\)<br/>
\\( U\_2(\mathbf p) = (0, 1, 0)\\)<br/>
\\( U\_3(\mathbf p) = (0, 0, 1)\\)
</p></div>

for each point \\( \mathbf p \in \mathbf R^3 \\). Collectively, they are called
the _natural frame field_ on \\( \mathbf R^3 \\).
