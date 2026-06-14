# Safety

## Layers

1. Must destroy all children before destroying the parent -> if you destroy the parent first, the child has a dangling pointer
2. Parent contains a reference to the first child, each child contains a reference to the next.
3. Windows have a root layer which should outlive all layers within it
4. Fonts should outlive any layer they're attached to
5. Fonts should be sharable between layers
6. Layer render callbacks need to be able to modify the layer and draw to it
7. Some things are global like visible windows, timers, etc
8. layer_destroy cannot be called after the parent is destroyed

I'm looking at wrapping a c library in a safe rust wrapper.

The base definitions look like:


```
void layer_remove_from_parent(Layer *child);
void layer_add_child(Layer *parent, Layer *child);
Layer* layer_create(GRect frame);
void layer_destroy(Layer* layer);
void layer_set_color(Layer* layer, GGolor color);
```

layer_destroy cannot be called after the parent is destroyed, so parents must outlive children.

My goal is that a complex tree can be created, specific mutable (or interior mutable) references can be retained, references that are not retained are not deleted, and remain in place in the tree.

When a node that doesn't have a parent goes out of scope, it should clean itself up and all children.

What would a safe rust wrapper look like that has these properties? 


Option 1: Lifetimes

You can only set the parent if the parent has a lifetime longer than the child.

Option 2: Rc

The parent won't be destructed as long as the child is still loaded
