# zombin

将一组数据在直角坐标系上显示时，需要确定纵轴的的最大最小取之范围，以及取值间隔。

假设这组数据在纵轴上的取值为数组{a1, a2, a3, ..., am}，以及取值间隔数 n（正整数）。

可以确定的是：
- 纵轴的最大最小值需要涵盖这组数据 y_max >= max{a1, a2, a3, ..., am}, y_min <= min{a1, a2, a3, ..., am}
- 纵轴的标度值的非零最低位需要尽可能大。

设：amax = max{a1, a2, a3, ..., am}, amin = min{a1, a2, a3, ..., am}，interval = (amax-amin)/n。
那么，y_max >= amax, y_min <= amin。

因此，y_interval = (y_max - y_min)/n

为了集中显示数据，规定 (y_max - y_min) - (amax - amin) <= y_interval

公式回代，得：n*(y_interval-interval) <= y_interval

再设 tail = y_interval-interval

那么 tail <= interval/(n-1)

最后设 accuracy为tail将首位非零数值设为1后将其余位置零的数值。例如，当tail取0.0314时，accuracy取0.01。

此时结论就很明朗了。
y_interval = interval - interval%accuracy + accuracy
y_min = amin - amin%accuracy
y_max = y_min + y_interval*n
