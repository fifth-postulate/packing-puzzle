# Determine images of the homomorphism between to a permutation group
# and a matrix groups both isomorphic to S_4.
G := Group((1, 2, 3, 4), (2, 3, 4));
H := Group([[0, -1, 0], [1, 0, 0], [0, 0, 1]], [[0, 1, 0], [0, 0, 1], [1, 0, 0]]);

hom := GroupHomomorphismByImages(G, H);

for g in G do
    l := [1..4];
    Apply(l, i -> String((i^g) - 1));
    w := Concatenation(l);
    image := Image(hom, g);
    Print(w, " ", image, "\n");
od;