import React from "react";
import * as lodash from "lodash";

const style = {
	background: "#00c",
	color: "#fff",
	padding: 12,
};

const Button = () => (
	<button type="button" style={style}>
		Guest Button - lodash {lodash.VERSION}
	</button>
);

export default Button;
