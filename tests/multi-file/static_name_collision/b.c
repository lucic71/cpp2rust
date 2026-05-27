static float same_name_different_type = 1.0f;
static int same_name_same_type = 6;

float b_foo() { return same_name_different_type; }
int b_bar() { return same_name_same_type; }
