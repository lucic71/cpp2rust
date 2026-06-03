typedef enum { WIDGET_A, WIDGET_B, WIDGET_C } widget;

int b_value(void) {
  widget w = WIDGET_C;
  return (int)w;
}
