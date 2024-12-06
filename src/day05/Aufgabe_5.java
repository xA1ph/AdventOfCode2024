package day05;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Aufgabe_5 {

	private static List<String> inputList = new ArrayList<>();
	private static List<String> inputList2 = new ArrayList<>();

	public static void main(String[] args) {
		Path path = Path.of("src/day05/Task05.txt");

		Pattern pattern = Pattern.compile("\\d{1,3}");
		Matcher matcher;

		int result = 0;
		int result2 = 0;

		try (Stream<String> input = Files.lines(path); Stream<String> input2 = Files.lines(path)) {
			inputList = input.collect(Collectors.toList());
			inputList2 = input2.collect(Collectors.toList());

			List<String> conditions = new ArrayList<>();
			List<String> updates = new ArrayList<>();
			List<Integer> updInt;
			List<List<Integer>> corrUpd = new ArrayList<>();
			List<List<Integer>> corrUpd2 = new ArrayList<>();

			conditions = condList(inputList);
			updates = updateList(inputList);

			for (String upd : updates) {
				updInt = new ArrayList<>();
				matcher = pattern.matcher(upd);
				while (matcher.find()) {
					updInt.add(Integer.valueOf(matcher.group()));
				}
				innerloop: for (String cond : conditions) {
					matcher = pattern.matcher(cond);
					String left = "";
					String right = "";
					if (matcher.find()) {
						left = matcher.group();
					}
					if (matcher.find()) {
						right = matcher.group();
					}
					if (updInt.contains(Integer.valueOf(left)) && updInt.contains(Integer.valueOf(right))) {
						int posLeft = 0;
						int posRight = 0;
						posLeft = updInt.indexOf(Integer.valueOf(left));
						posRight = updInt.indexOf(Integer.valueOf(right));
						if ((posLeft >= 0) && (posRight >= 0) && posLeft < posRight) {
							if (!(corrUpd.contains(updInt))) {
								corrUpd.add(updInt);
							}
						} else {
							if (corrUpd.contains(updInt)) {
								corrUpd.remove(updInt);
							}
							break innerloop;
						}
					}
				}

			}

			for (List<Integer> z : corrUpd) {
				result += getMid(z);
			}

			System.out.println(result);

			// ------------------------------//

			List<List<Integer>> falseUpd = new ArrayList<>();

			for (String upd : updates) {
				updInt = new ArrayList<>();
				matcher = pattern.matcher(upd);
				while (matcher.find()) {
					updInt.add(Integer.valueOf(matcher.group()));
				}
				innerloop: for (String cond : conditions) {
					matcher = pattern.matcher(cond);
					String left = "";
					String right = "";
					if (matcher.find()) {
						left = matcher.group();
					}
					if (matcher.find()) {
						right = matcher.group();
					}
					if (updInt.contains(Integer.valueOf(left)) && updInt.contains(Integer.valueOf(right))) {
						int posLeft = 0;
						int posRight = 0;
						posLeft = updInt.indexOf(Integer.valueOf(left));
						posRight = updInt.indexOf(Integer.valueOf(right));
						if ((posLeft >= 0) && (posRight >= 0) && posLeft < posRight) {

						} else {
							if (!falseUpd.contains(updInt)) {
								falseUpd.add(updInt);
							}
							break innerloop;
						}
					}
				}

			}
			int i = 0;

			for (List<Integer> updInt2 : falseUpd) {
				for (String cond2 : conditions) {
					boolean passt = false;
					while (!passt) {
						innerloop: for (String cond : conditions) {
							matcher = pattern.matcher(cond);
							String left = "";
							String right = "";
							if (matcher.find()) {
								left = matcher.group();
							}
							if (matcher.find()) {
								right = matcher.group();
							}
							if (updInt2.contains(Integer.valueOf(left)) && updInt2.contains(Integer.valueOf(right))) {
								int posLeft = 0;
								int posRight = 0;
								posLeft = updInt2.indexOf(Integer.valueOf(left));
								posRight = updInt2.indexOf(Integer.valueOf(right));
								if ((posLeft >= 0) && (posRight >= 0) && posLeft < posRight) {
									if (!(corrUpd2.contains(updInt2))) {
										corrUpd2.add(updInt2);
									}
									if (i >= conditions.size()) {
										passt = true;
									}
								} else {
									if ((corrUpd2.contains(updInt2))) {
										corrUpd2.remove(updInt2);
									}
									int tmp = updInt2.get(posLeft);
									int tmp2 = updInt2.get(posRight);
									updInt2.set(posRight, tmp);
									updInt2.set(posLeft, tmp2);
									if (!(corrUpd2.contains(updInt2))) {
										corrUpd2.add(updInt2);
									}
									i = 0;
									break innerloop;
								}
							}
							i++;
						}
					}

				}

			}

			for (List<Integer> z : corrUpd2) {
				result2 += getMid(z);
			}

			System.out.println(result2);

		} catch (IOException e) {
			System.out.println("Error");
		}

	}

	private static ArrayList<String> condList(List<String> list) {
		ArrayList<String> conditions = new ArrayList<>();
		for (String line : inputList) {
			if ("".equals(line)) {
				break;
			} else {
				conditions.add(line);
			}
		}
		return conditions;
	}

	private static ArrayList<String> updateList(List<String> list) {
		ArrayList<String> updates = new ArrayList<String>();
		for (int i = 0; i < list.size(); i++) {
			if (i > 1176) {
				updates.add(list.get(i));
			}
		}
		return updates;
	}

	private static int getMid(List<Integer> list) {
		int index = list.size() / 2;
		return list.get(index);
	}

}
